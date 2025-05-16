use cubecl::prelude::*;

#[cube(launch)]
fn blur(input: &Array<f32>, output: &mut Array<f32>, width: u32, height: u32, blur_size: i32) {
    let (x, y) = (ABSOLUTE_POS_X, ABSOLUTE_POS_Y);

    if x < width && y < height {
        let mut num_pixels: u32 = 0;
        let mut sum_pixels: f32 = 0.;

        for blur_row in -blur_size..=blur_size {
            for blur_col in -blur_size..=blur_size {
                let curr_row = y as i32 + blur_row;
                let curr_col = x as i32 + blur_col;

                if curr_row >= 0
                    && (curr_row as u32) < height
                    && curr_col >= 0
                    && (curr_col as u32) < width
                {
                    sum_pixels = sum_pixels + input[curr_row as u32 * width + curr_col as u32];
                    num_pixels += 1;
                }
            }
        }

        output[ABSOLUTE_POS] = sum_pixels / (num_pixels as f32);
    }
}

pub fn launch<R: Runtime>(device: &R::Device, input: &Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let client = R::client(device);
    let vectorization = 1;

    let tile = 20;
    let groups_x = (width + tile - 1) / tile;
    let groups_y = (height + tile - 1) / tile;

    let input: Vec<f32> = input.iter().map(|&b| b as f32).collect();

    let output_handle = client.empty(input.len() * core::mem::size_of::<f32>());
    let input_handle = client.create(f32::as_bytes(&input));

    unsafe {
        blur::launch::<R>(
            &client,
            CubeCount::Static(groups_x, groups_y, 1),
            CubeDim::new(tile, tile, 1),
            ArrayArg::from_raw_parts::<f32>(&input_handle, input.len(), vectorization),
            ArrayArg::from_raw_parts::<f32>(&output_handle, input.len(), vectorization),
            ScalarArg::new(width),
            ScalarArg::new(height),
            ScalarArg::new(12),
        )
    }

    let bytes = client.read_one(output_handle.binding());
    let output = f32::from_bytes(&bytes);
    output.to_vec().iter().map(|&b| b as u8).collect()
}

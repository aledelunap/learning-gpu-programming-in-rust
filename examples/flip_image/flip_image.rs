use cubecl::prelude::*;

#[cube(launch)]
fn flip<I: Int>(input: &Array<I>, output: &mut Array<I>, width: u32, height: u32) {
    let (x, y) = (ABSOLUTE_POS_X, ABSOLUTE_POS_Y);

    if x < width && y < height {
        let index = y * width + x;
        let flip_x = (width as i32 - 1 - x as i32) as u32;
        let flip_index = y * width + flip_x;

        output[flip_index as u32] = input[index as u32];
    }
}

pub fn launch<R: Runtime>(device: &R::Device, input: &Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let client = R::client(device);
    let vectorization = 1;

    let tile = 20;
    let groups_x = (width + tile - 1) / tile;
    let groups_y = (height + tile - 1) / tile;

    let input: Vec<u32> = input.iter().map(|&b| b as u32).collect();

    let output_handle = client.empty(input.len() * core::mem::size_of::<u32>());
    let input_handle = client.create(u32::as_bytes(&input));

    unsafe {
        flip::launch::<u32, R>(
            &client,
            CubeCount::Static(groups_x, groups_y, 1),
            CubeDim::new(tile, tile, 1),
            ArrayArg::from_raw_parts::<u32>(&input_handle, input.len(), vectorization),
            ArrayArg::from_raw_parts::<u32>(&output_handle, input.len(), vectorization),
            ScalarArg::new(width),
            ScalarArg::new(height),
        )
    };

    let bytes = client.read_one(output_handle.binding());
    let output = u32::from_bytes(&bytes);
    output.to_vec().iter().map(|&b| b as u8).collect()
}

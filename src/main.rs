use std::fs;
use rsp_client_executor::{
    io::ClientExecutorInput, ClientExecutor, ChainVariant, EthereumVariant, CHAIN_ID_ETH_MAINNET, CHAIN_ID_LINEA_MAINNET,
    CHAIN_ID_OP_MAINNET,
};
use bincode;

fn main() {

    let f = fs::read("input/1/18884864.bin").expect("could not read file");

    // load the prev state + new block
    let input1: ClientExecutorInput = bincode::deserialize(&f).expect("could not deserialize");

    // Make a copy of the input and change tx #16 to be a copy of #17.
    // This will test if revm can still execute the block, even with some invalid transactions
    let mut input2 = input1.clone();
    input2.current_block.body[16] = input2.current_block.body[15].clone();

    let executor = ClientExecutor;
    println!("block1 success: {}", executor.execute::<EthereumVariant>(input1).is_ok());
    println!("block2 result: {:?}", executor.execute::<EthereumVariant>(input2).unwrap());
}
/**
 * This is a stand-alone script that deploys the
 * L2 bridge with fuels-ts sdk.
 */

import abi from "./out/release/my-fuel-project-abi.json";
import slots from "./out/release/my-fuel-project-storage_slots.json";
import fs from "fs";

import { hexlify, sha256, ContractFactory, Wallet } from "fuels";
const L2_RPC = "http://localhost:4000/v1/graphql";
const L2_SIGNER =
  "0xde97d8624a438121b86a1956544bd72ed68cd69f2c99555b08b1e8c51ffd511c";

const main = async () => {
  const binary = hexlify(fs.readFileSync("./out/release/my-fuel-project.bin"));
  console.log("bytecode sha", sha256(binary));

  // const wallet = Wallet.fromPrivateKey(L2_SIGNER);

  // const contract = new ContractFactory(binary, abi)
  // console.log("Contract ID", contract.id.toB256());
  // contract.setConfigurableConstants({BRIDGED_TOKEN_GATEWAY: })
};

main()
  .then(() => {
    console.log("\t> Finished");
    process.exit(0);
  })
  .catch((e) => {
    console.error(e);
    process.exit(1);
  });

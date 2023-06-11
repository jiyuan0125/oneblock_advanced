import { ApiPromise } from "@polkadot/api";

async function main() {
  // Create our API with a default connection to the local node
  const api = await ApiPromise.create();

  // 获取 Offchain 数据
  const offchain = await api.rpc.offchain.localStorageGet("PERSISTENT", "kuaidi100::indexing_parcel_weight");

  console.log(offchain.toHex());

}

main().catch((error) => {
  console.error(error);
  process.exit(-1);
});
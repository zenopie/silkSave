import { SecretNetworkClient, Wallet, MsgExecuteContract} from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm.gz");

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-2",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});


const sscrt_contract = "secret1c9dfnjl6lnhjr5cqmtfnfqej87uspcltkg0d0l";
const sscrt_hash = "593c95b45c9d034148c4dcc02b858314a841e477cedbeb72d33b25caa0786af2";
const codeId = "21246";
const contractCodeHash = "43538d55e9431e447c5e933694ac55ce830d7b2dd10f39a6c49a770998e2c2c8";
const contract_address = "secret138kk7ps76k4v58jss98wmgpf7nzk4xnwtgcdfp";





let upload_contract = async () => {
  let tx = await secretjs.tx.compute.storeCode(
    {
      sender: wallet.address,
      wasm_byte_code: contract_wasm,
      source: "",
      builder: "",
    },
    {
      gasLimit: 4_000_000,
    }
  );

  const codeId = Number(
    tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
      .value
  );

  console.log("codeId: ", codeId);

  const contractCodeHash = (
    await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
  ).code_hash;
  console.log(`Contract hash: ${contractCodeHash}`);
  
};

//upload_contract();





let instantiate_contract = async () => {
  // Create an instance of the Counter contract, providing a starting count
  const initMsg = {
     token: sscrt_contract,
     hash: sscrt_hash, 
    };
  let tx = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: contractCodeHash,
      init_msg: initMsg,
      label: "🌎" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );


  //Find the contract_address in the logs
  const contractAddress = tx.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  ).value;

  console.log(contractAddress);

};

//instantiate_contract();



















let try_test = async () => {
  const my_query = await secretjs.query.compute.queryContract({
    contract_address: contract_address,
    code_hash: contractCodeHash,
    query: { test: {} },
  });

  console.log(my_query);
};

let try_query_balance = async () => {
  const my_query = await secretjs.query.compute.queryContract({
    contract_address: contract_address,
    code_hash: contractCodeHash,
    query: { get_balance: {} },
  });

  console.log(my_query);
};

let try_query_stake = async () => {
  const my_query = await secretjs.query.compute.queryContract({
    contract_address: contract_address,
    code_hash: contractCodeHash,
    query: { 
      get_stake: { 
        address: wallet.address,
      },
    },
  });

  console.log(my_query);
};

async function sendSscrt(){
  let hookmsg = {
    stake: {}
  }
  let hookmsg64 = Buffer.from(JSON.stringify(hookmsg)).toString("base64");
	let msg = new MsgExecuteContract({
		sender: secretjs.address,
		contract_address: sscrt_contract,
		code_hash: sscrt_hash,
		msg: {
			send: {
				recipient: contract_address,
				recipient_code_hash: contractCodeHash,
				amount: "1000000",
        msg: hookmsg64,
			}
		}
	});
	let resp = await secretjs.tx.broadcast([msg], {
		gasLimit: 1_000_000,
		gasPriceInFeeDenom: 0.1,
		feeDenom: "uscrt",
	});
	console.log(resp);
};

let try_withdraw = async () => {
  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contract_address,
      code_hash: contractCodeHash, // optional but way faster
      msg: {
        withdraw: {
          token: sscrt_contract,
          hash: sscrt_hash,
          amount: "500000",
        },
      },
    },
    {
      gasLimit: 1_000_000,
    }
  );
  console.log(tx);
};

//try_query_balance();
//try_query_stake();
//sendSscrt();
//try_test();
//try_withdraw();

<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="2"></v-col>
      <v-col cols="8">
        <v-row>
          <v-col>
            <v-text-field v-model="player" label="Player"></v-text-field>
          </v-col>
          <v-col>
            <v-text-field v-model="data" label="Data"></v-text-field>
          </v-col>
        </v-row>
        <v-row>
          <v-col></v-col>
          <v-col>
            <v-btn rounded @click="read">Read</v-btn>
            <v-btn rounded @click="send">Save</v-btn>
          </v-col>
          <v-col></v-col>
        </v-row>
        <div style="height: 20px"></div>
        <v-row>
          {{ this.readResponse }}
        </v-row>
      </v-col>
      <v-col cols="2"></v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { Vue, Component } from 'vue-property-decorator';
import { SigningCosmWasmClient, Secp256k1HdWallet, GasPrice, Decimal, OfflineSigner, AccountData } from "cosmwasm";

@Component
export default class Demo extends Vue {
  rpcEndpoint = "http://localhost:26657";

  mnemonic =
      "honey barely alien maid brave dash nothing panel there own swarm neglect alert faculty violin blade bench cushion daring fun category chief village october";

  wallet = {} as OfflineSigner;

  client = {} as SigningCosmWasmClient;

  account = {} as AccountData;

  contractAddress = "cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr";

  player = "";
  data = "";

  readResponse = "";

  async mounted() {
    this.wallet = await Secp256k1HdWallet.fromMnemonic(this.mnemonic);

    this.client = await SigningCosmWasmClient.connectWithSigner(
        this.rpcEndpoint,
        this.wallet,
        {
          gasPrice: new GasPrice(Decimal.fromUserInput("0", 2), "stake"),
        }
    );

    const [first] = await this.wallet.getAccounts();
    this.account = first;
  }

  async send() {
    const resp = await this.client.execute(
        this.account.address,
        this.contractAddress,
        {
          "add": {
            "player": this.player,
            "data": this.data
          }
        },
        "auto"
    );

    console.log(resp);
  }

  async read() {
    const resp = await this.client.queryContractSmart(
        this.contractAddress,
        {
          "get_p_data": {
            "p": this.player
          }
        }
    );
    // console.log(resp);
    // this.readResponse = resp;

    try {
      let data = await fetch(`http://${resp.pdata}`);
      // console.log(data)
      let z = await data.json();
      // console.log(z);
      this.readResponse = z;
    } catch (e) {
      this.readResponse = "ERRORU!";
    }
  }

}
</script>

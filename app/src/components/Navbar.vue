<script setup>
import {defineProps} from 'vue'
import Button from './forms/Button.vue';

const props = defineProps({
  title: {type: String, required: true},
  links: {type: Array, required: true}
});
</script>

<template>
<div class="fixed left-0 right-0 top-0 h-16 shadow-md border-b-2 border-gray-100 bg-gray-900">
  <nav class="flex items-center container mx-auto h-full justify-between">
    <h1 class="font-semibold uppercase text-lg text-gray-200">
      BitWrite
    </h1>
    <div>
      <ul class="flex items-center space-x-10 text-sm">
        <li v-for="link in props.links" :key="link.id">
          <router-link :to="link.path" class="text-gray-400 hover:text-gray-100">
            {{link.name}}
          </router-link>
        </li>
      </ul>
    </div>
    <div>
      <Button :text="isConnected ? 'Disconnect' : 'Connect'" @click="onSumbit()"/>
      <p class="text-gray-400">{{pubKey}}</p>
    </div>
  </nav>
</div>
</template>

<script>
import { getConnection, useWallet } from "../solana/connection";
import {
  Account,
  clusterApiUrl,
  Connection,
  PublicKey,
  SystemProgram,
  Transaction
} from "@solana/web3.js";
import Wallet from "@project-serum/sol-wallet-adapter";
const walletMap = {
  sollet: "https://www.sollet.io",
  bonfida: "https://www.bonfida.com/wallet",
  solflare: "https://www.solflare.com"
};
export default {
  data: () => ({
    wallets: ["sollet", "bonfida", "solflare"],
    wallet: "sollet",
    pubKey: "",
    isConnected: false
  }),

  methods: {
    async onSumbit() {
      try {
        const wallet = await useWallet()

        if (this.isConnected) {
          await wallet.disconnect()
          this.isConnected = false
          this.pubKey = ''
          return
        }
        // const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
        // const wallet = new Wallet(walletMap[this.wallet], "devnet");
        // await wallet.connect();
        this.pubKey = wallet.publicKey
        
        this.isConnected = true
      } catch (err) {
        console.log("Something went worng :(")
      }
    }
  }
}
</script>
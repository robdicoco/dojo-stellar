<template>
  <div>
    <h2>Search Transaction by Hash</h2>
    <input v-model="transactionHash" placeholder="Enter transaction hash" />
    <button @click="searchTransaction">Search</button>
    <div v-if="transaction">
      <pre>{{ transaction }}</pre>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      transactionHash: '',
      transaction: null,
    };
  },
  methods: {
    async searchTransaction() {
      try {
        const response = await axios.get(`http://localhost:8000/transaction/${this.transactionHash}`);
        this.transaction = response.data;
      } catch (error) {
        alert('Transaction not found');
      }
    },
  },
};
</script>

<template>
  <div>
    <h2>Search Balance by Address</h2>
    <input v-model="address" placeholder="Enter address" />
    <button @click="searchBalance">Search</button>
    <div v-if="balance">
      <pre>{{ balance }}</pre>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      address: '',
      balance: null,
    };
  },
  methods: {
    async searchBalance() {
      try {
        const response = await axios.get(`http://localhost:8000/balance/${this.address}`);
        this.balance = response.data;
      } catch (error) {
        alert('Address not found');
      }
    },
  },
};
</script>

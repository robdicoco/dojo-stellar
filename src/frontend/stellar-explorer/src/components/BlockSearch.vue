<template>
  <div>
    <h2>Search Block by Number</h2>
    <input v-model="blockNumber" placeholder="Enter block number" />
    <button @click="searchBlock">Search</button>
    <div v-if="block">
      <pre>{{ block }}</pre>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  data() {
    return {
      blockNumber: '',
      block: null,
    }
  },
  methods: {
    async searchBlock() {
      try {
        const response = await axios.get(`http://localhost:8000/block/${this.blockNumber}`)
        this.block = response.data
      } catch (error) {
        alert('Block not found')
      }
    },
  },
}
</script>

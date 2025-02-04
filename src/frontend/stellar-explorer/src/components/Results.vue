<template>
  <div v-if="result" class="results">
    <h2>{{ title }}</h2>
    <div v-if="error" class="error">
      <p>Error: {{ error }}</p>
    </div>
    <div v-else class="content">
      <div v-if="result.result === 'ledger_info'">
        <p><strong>Ledger Sequence:</strong> {{ result.data.sequence }}</p>
        <p><strong>Closed At:</strong> {{ result.data.closed_at }}</p>
        <!-- Add more fields as needed -->
      </div>
      <div v-else-if="result.result === 'transaction_info'">
        <p><strong>Transaction Hash:</strong> {{ result.data.hash }}</p>
        <p><strong>Status:</strong> {{ result.data.successful ? 'Success' : 'Failed' }}</p>
        <p><strong>Source Account:</strong> {{ result.data.source_account }}</p>
        <p><strong>Created At:</strong> {{ result.data.created_at }}</p>
        <!-- Add more fields as needed -->
      </div>
      <div v-else-if="result.result === 'account_info'">
        <p><strong>Account ID:</strong> {{ result.data.account_id }}</p>
        <p><strong>Balance:</strong> {{ result.data.balance }}</p>
        <!-- Add more fields as needed -->
      </div>
      <div v-else>
        <p>No results found.</p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Results',
  props: {
    result: Object,
  },
  computed: {
    title() {
      if (this.result) {
        return this.result.result.replace('_', ' ').replace('info', ' Information').toUpperCase()
      }
      return ''
    },
    error() {
      return this.result && this.result.detail
    },
  },
}
</script>

<style scoped>
.results {
  margin-top: 20px;
}

.error {
  color: red;
}

.content {
  background-color: #2e2e2e;
  padding: 10px;
  border-radius: 5px;
}

.content p {
  margin: 5px 0;
}
</style>

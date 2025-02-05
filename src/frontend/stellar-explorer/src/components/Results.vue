<template>
  <div v-if="result" class="results">
    <h2>{{ title }}</h2>
    <div v-if="error" class="error">
      <p>Error: {{ error }}</p>
    </div>
    <div v-else class="content">
      <div v-if="result.result === 'ledger_info'">
        <p><strong>id:</strong> {{ result.data.id }}</p>
        <p><strong>Ledger Sequence:</strong> {{ result.data.sequence }}</p>
        <p><strong>Previous Id:</strong> {{ result.data.prev_hash }}</p>
        <p><strong>Closed At:</strong> {{ result.data.closed_at }}</p>
        <p><strong>Protocol Version:</strong> {{ result.data.protocol_version }}</p>
        <p><strong>Operations:</strong> {{ result.data.operation_count }}</p>
        <p><strong>Success Transactions:</strong> {{ result.data.successful_transaction_count }}</p>
        <p><strong>Failed Transactions:</strong> {{ result.data.failed_transaction_count }}</p>
        <p><strong>XDR Data:</strong></p>
        <textarea readonly rows="8" cols="80">
            {{ result.data.header_xdr }}
        </textarea>
      </div>
      <div v-else-if="result.result === 'transaction_info'">
        <p><strong>Transaction Hash:</strong> {{ result.data.hash }}</p>
        <p><strong>Status:</strong> {{ result.data.successful ? 'Success' : 'Failed' }}</p>
        <p><strong>Source Account:</strong> {{ result.data.source_account }}</p>
        <p><strong>Created At:</strong> {{ result.data.created_at }}</p>
        <p><strong>Operations:</strong> {{ result.data.operation_count }}</p>
        <p><strong>Source Account:</strong> {{ result.data.source_account }}</p>
        <p><strong>Fee Charged:</strong> {{ result.data.fee_charged }}</p>
        <p><strong>Ledger:</strong> {{ result.data.ledger }}</p>
        <p><strong>XDR Envelope:</strong></p>
        <textarea readonly rows="8" cols="80">
            {{ result.data.envelope_xdr }}
        </textarea>
        <p><strong>XDR Result:</strong></p>
        <textarea readonly rows="1" cols="80">
            {{ result.data.result_xdr }}
        </textarea>
        <p><strong>XDR Result Meta:</strong></p>
        <textarea readonly rows="8" cols="80">
            {{ result.data.result_meta_xdr }}
        </textarea>
        <p><strong>XDR Fee Meta:</strong></p>
        <textarea readonly rows="3" cols="80">
            {{ result.data.fee_meta_xdr }}
        </textarea>
      </div>
      <div v-else-if="result.result === 'account_info'">
        <!-- <p><strong>Asset Type:</strong> {{ result.data.asset_type }}</p>
        <p><strong>Balance:</strong> {{ data.balance }}</p> -->
        <table border="1" cellpadding="10">
          <thead>
            <tr>
              <th>Asset Type</th>
              <th>Balance</th>
              <th>Buying Liabilities</th>
              <th>Selling Liabilities</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(item, index) in result.data" :key="index">
              <td>{{ item.asset_type }}</td>
              <td>{{ item.balance }}</td>
              <td>{{ item.buying_liabilities }}</td>
              <td>{{ item.selling_liabilities }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-else>
        <p>No results found.</p>
        <p><strong>Detail:</strong> {{ result.detail }}</p>
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
      try {
        if (this.result) {
          return this.result.result.replace('_', ' ').replace('info', ' Information').toUpperCase()
        }
      } catch (error) {
        console.info('Detail:', error)
        return ''
      }
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
  background-color: var(--background-color);
  padding: 10px;
  border-radius: 5px;
}

.content p {
  margin: 5px 0;
}

.xdr {
  width: 100%;
  height: 300px;
  overflow: auto;
  border: 1px solid #ccc;
  padding: 10px;
}

table {
  width: 100%;
  border-collapse: collapse;
}
th,
td {
  text-align: left;
  padding: 8px;
}
th {
  background-color: var(--background-color);
}
</style>

<template>
  <div class="latest-ledgers">
    <h2>Latest Ledgers</h2>
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>Hash</th>
          <th>Sequence</th>
          <th>Operations</th>
          <th>Closed At</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(ledger, index) in ledgers" :key="index">
          <td>{{ ledger.id }}</td>
          <td>{{ ledger.hash }}</td>
          <td>{{ ledger.sequence }}</td>
          <td>{{ ledger.operations }}</td>
          <td>{{ ledger.closedAt }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  name: 'LatestLedgers',
  data() {
    return {
      // Sample ledger data
      ledgers: [
        {
          id: '55547719',
          hash: 'sed39154...18b5465e',
          sequence: 22,
          operations: 0,
          closedAt: '2025-02-03 AM 3:33:51',
        },
        {
          id: '55547720',
          hash: 'abc12345...67890xyz',
          sequence: 23,
          operations: 1,
          closedAt: '2025-02-03 AM 3:34:00',
        },
        // Add more ledger entries as needed
      ],
    }
  },
}
</script>

<style scoped>
.latest-ledgers {
  margin: 20px 0;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 10px;
  text-align: left;
  border-bottom: 1px solid #333;
}

th {
  background-color: #222;
  color: #aaa;
  font-weight: bold;
}

td {
  background-color: #1e1e1e;
  color: #ffffff;
}

@media (max-width: 768px) {
  table {
    font-size: 14px;
  }
}
</style>

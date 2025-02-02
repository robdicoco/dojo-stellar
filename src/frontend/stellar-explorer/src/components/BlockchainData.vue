<template>
  <div class="blockchain-data">
    <!-- Left Column -->
    <div class="column left-column">
      <div class="metric" v-for="(metric, index) in leftMetrics" :key="index">
        <p class="label">{{ metric.label }}</p>
        <p class="value">{{ metric.value }}</p>
      </div>
    </div>

    <!-- Right Column -->
    <div class="column right-column">
      <div class="metric" v-for="(metric, index) in rightMetrics" :key="index">
        <p class="label">{{ metric.label }}</p>
        <p class="value">{{ metric.value }}</p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'BlockchainData',
  data() {
    return {
      // Metrics for the left column
      leftMetrics: [
        { label: 'Total Accounts', value: '8,847,022,699' },
        { label: 'Total Assets', value: '336,371' },
        { label: 'XLM in Circulation', value: '30,576,001,864' },
        { label: 'TPS (24h)', value: '0' },
        { label: 'Average Ledger Time', value: '0.05' },
        { label: 'Protocol Version', value: '0' },
      ],
      // Metrics for the right column
      rightMetrics: [
        { label: 'Ledgers', value: '0' },
        { label: 'Transactions', value: '0' },
        { label: 'Operations', value: '0' },
        { label: 'Payments', value: '0' },
        { label: 'DEX Trades', value: '1,448,237' },
        { label: 'DEX Volume', value: '$3,565,379' },
      ],
    }
  },
}
</script>

<style scoped>
/* General Blockchain Data Styles */
.blockchain-data {
  display: flex;
  justify-content: space-between;
  margin: 20px 0;
}

/* Column Styles */
.column {
  width: 48%; /* Two columns with spacing between them */
}

.left-column {
  margin-right: 2%;
}

.right-column {
  margin-left: 2%;
}

/* Metric Styles */
.metric {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid #333; /* Separator line */
}

.metric:last-child {
  border-bottom: none; /* Remove border from the last metric */
}

/* Label and Value Styles */
.label {
  font-size: 14px;
  color: #aaa; /* Light gray for labels */
  text-transform: uppercase;
}

.value {
  font-size: 16px;
  font-weight: bold;
  color: #ffffff; /* White for values */
}

/* Responsive Design */
@media (max-width: 768px) {
  .blockchain-data {
    flex-direction: column;
  }

  .column {
    width: 100%;
    margin: 0;
  }

  .left-column {
    margin-bottom: 20px;
  }
}
</style>

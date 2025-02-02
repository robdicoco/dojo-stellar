<template>
  <div class="charts">
    <!-- Price Chart -->
    <div class="chart-container">
      <h2>Price</h2>
      <div class="time-period-tabs">
        <button
          v-for="(tab, index) in priceTabs"
          :key="index"
          :class="{ active: selectedPriceTab === tab }"
          @click="selectedPriceTab = tab"
        >
          {{ tab }}
        </button>
      </div>
      <apexchart
        type="area"
        height="350"
        :options="priceChartOptions"
        :series="priceSeries"
      ></apexchart>
    </div>

    <!-- Operations Chart -->
    <div class="chart-container">
      <h2>Operations</h2>
      <div class="time-period-tabs">
        <button
          v-for="(tab, index) in operationsTabs"
          :key="index"
          :class="{ active: selectedOperationsTab === tab }"
          @click="selectedOperationsTab = tab"
        >
          {{ tab }}
        </button>
      </div>
      <apexchart
        type="bar"
        height="350"
        :options="operationsChartOptions"
        :series="operationsSeries"
      ></apexchart>
    </div>

    <!-- Transactions Chart -->
    <div class="chart-container">
      <h2>Transactions</h2>
      <div class="time-period-tabs">
        <button
          v-for="(tab, index) in transactionsTabs"
          :key="index"
          :class="{ active: selectedTransactionsTab === tab }"
          @click="selectedTransactionsTab = tab"
        >
          {{ tab }}
        </button>
      </div>
      <apexchart
        type="bar"
        height="350"
        :options="transactionsChartOptions"
        :series="transactionsSeries"
      ></apexchart>
    </div>
  </div>
</template>

<script>
import ApexCharts from 'vue3-apexcharts'

export default {
  name: 'Charts',
  components: {
    apexchart: ApexCharts,
  },
  data() {
    return {
      // Time period tabs for each chart
      priceTabs: ['1D', '1W', '1M', '1Y'],
      operationsTabs: ['1D', '1W', '1M', '1Y'],
      transactionsTabs: ['1D', '1W', '1M', '1Y'],

      // Selected time periods
      selectedPriceTab: '1D',
      selectedOperationsTab: '1D',
      selectedTransactionsTab: '1D',

      // Price Chart Data
      priceChartOptions: {
        chart: {
          toolbar: {
            show: false,
          },
        },
        xaxis: {
          type: 'datetime',
        },
        yaxis: {
          labels: {
            formatter: function (val) {
              return '$' + val.toFixed(2)
            },
          },
        },
      },
      priceSeries: [
        {
          name: 'Price',
          data: this.generateData(),
        },
      ],

      // Operations Chart Data
      operationsChartOptions: {
        chart: {
          toolbar: {
            show: false,
          },
        },
        xaxis: {
          type: 'datetime',
        },
      },
      operationsSeries: [
        {
          name: 'Operations',
          data: this.generateData(),
        },
      ],

      // Transactions Chart Data
      transactionsChartOptions: {
        chart: {
          toolbar: {
            show: false,
          },
        },
        xaxis: {
          type: 'datetime',
        },
      },
      transactionsSeries: [
        {
          name: 'Transactions',
          data: this.generateData(),
        },
      ],
    }
  },
  methods: {
    generateData() {
      const data = []
      const now = new Date().getTime()
      for (let i = 0; i < 30; i++) {
        const timestamp = now - i * 24 * 60 * 60 * 1000
        const value = Math.floor(Math.random() * 100)
        data.push([timestamp, value])
      }
      return data
    },
  },
}
</script>

<style scoped>
/* General Charts Styles */
.charts {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
}

.chart-container {
  width: calc(33% - 20px); /* Three charts per row with spacing */
  margin-bottom: 20px;
}

@media (max-width: 768px) {
  .chart-container {
    width: 100%; /* One chart per row on medium screens */
    margin-bottom: 20px;
  }
}

/* Chart Title and Tabs */
h2 {
  font-size: 24px;
  color: #ffffff;
  margin-bottom: 10px;
}

.time-period-tabs {
  display: flex;
  margin-bottom: 10px;
}

.time-period-tabs button {
  background-color: #333;
  color: #aaa;
  border: none;
  padding: 5px 10px;
  margin-right: 5px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.time-period-tabs button.active {
  background-color: #00bcd4;
  color: #ffffff;
}
</style>

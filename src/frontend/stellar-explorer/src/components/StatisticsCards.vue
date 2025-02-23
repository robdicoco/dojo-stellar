<template>
  <div class="statistics-cards">
    <div class="card" v-for="(crypto, index) in cryptos" :key="index">
      <div class="icon">
        <i :class="getIcon(crypto.symbol)"></i>
      </div>
      <div class="data">
        <p class="value">{{ crypto.name }} ({{ crypto.symbol }})</p>
        <p class="label">Rank: {{ crypto.rank }}</p>
        <p class="label">Price: ${{ crypto.price.toFixed(2) }}</p>
        <p class="label">Market Cap: ${{ formatNumber(crypto.market_cap) }}</p>
        <p class="label">24h Volume: ${{ formatNumber(crypto.volume_24h) }}</p>
      </div>
      <div class="percentage" :class="getPercentageClass(crypto.percent_change_24h)">
        <span>{{ crypto.percent_change_24h.toFixed(2) }}%</span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatisticsCards',
  data() {
    return {
      cryptos: [], // Array to store cryptocurrency data
    }
  },
  async created() {
    // Fetch data from the API when the component is created
    await this.fetchData()
  },
  methods: {
    async fetchData() {
      try {
        const response = await fetch('api/latest_data')
        const data = await response.json()
        this.cryptos = data[0] // Extract the first array from the response
      } catch (error) {
        console.error('Error fetching data:', error)
      }
    },
    getIcon(symbol) {
      // Map cryptocurrency symbols to icons (you can customize this)
      const icons = {
        BTC: 'fab fa-btc',
        ETH: 'fab fa-ethereum',
        XRP: 'fas fa-coins',
        USDT: 'fas fa-dollar-sign',
        BNB: 'fab fa-bnb',
        SOL: 'fas fa-sun',
        USDC: 'fas fa-dollar-sign',
        DOGE: 'fab fa-dogecoin',
        ADA: 'fab fa-ada',
        TRX: 'fab fa-tron',
        XLM: 'fas fa-star',
      }
      return icons[symbol] || 'fas fa-coins' // Default icon
    },
    formatNumber(number) {
      // Format large numbers with commas
      return number.toLocaleString()
    },
    getPercentageClass(percentage) {
      if (percentage > 0) return 'positive'
      if (percentage < 0) return 'negative'
      return 'neutral'
    },
  },
}
</script>

<style scoped>
.statistics-cards {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  margin: 20px 0;
}

.card {
  display: flex;
  align-items: center;
  background-color: var(--background-color);
  padding: 15px;
  border-radius: 10px;
  width: calc(25% - 20px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease;
  margin-bottom: 20px;
}

.card:hover {
  transform: translateY(-5px);
}

.icon {
  background-color: var(--background-color);
  color: var(--text-color);
  padding: 10px;
  border-radius: 50%;
  font-size: 24px;
  margin-right: 15px;
}

.data {
  flex-grow: 1;
}

.value {
  font-size: 18px;
  font-weight: bold;
  color: var(--text-color);
  margin-bottom: 5px;
}

.label {
  font-size: 14px;
  color: #aaa;
  margin-bottom: 3px;
}

.percentage {
  font-size: 14px;
  font-weight: bold;
  padding: 5px 10px;
  border-radius: 5px;
}

.positive {
  color: #4caf50;
  background-color: rgba(76, 175, 80, 0.2);
}

.negative {
  color: #f44336;
  background-color: rgba(244, 67, 54, 0.2);
}

.neutral {
  color: #aaa;
  background-color: rgba(170, 170, 170, 0.2);
}

@media (max-width: 768px) {
  .card {
    width: calc(50% - 20px);
  }
}

@media (max-width: 480px) {
  .card {
    width: 100%;
  }
}
</style>

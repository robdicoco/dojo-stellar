<template>
  <div class="statistics-cards">
    <div class="card" v-for="(card, index) in cards" :key="index">
      <div class="icon">
        <i :class="card.icon"></i>
      </div>
      <div class="data">
        <p class="value">{{ card.value }}</p>
        <p class="label">{{ card.label }}</p>
      </div>
      <div class="percentage" :class="getPercentageClass(card.percentage)">
        <span>{{ card.percentage }}%</span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'StatisticsCards',
  data() {
    return {
      cards: [
        {
          icon: 'fas fa-hashtag', // Rank icon
          value: '13',
          label: 'RANK',
          percentage: '0',
        },
        {
          icon: 'fas fa-dollar-sign', // Price icon
          value: '$0.3933',
          label: 'PRICE',
          percentage: '0',
        },
        {
          icon: 'fas fa-chart-line', // Market Cap icon
          value: '$12,025,964,968',
          label: 'MARKET CAP',
          percentage: '0',
        },
        {
          icon: 'fas fa-exchange-alt', // 24h Volume icon
          value: '$268,861,566',
          label: '24H VOLUME',
          percentage: '0',
        },
      ],
    }
  },
  methods: {
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
  background-color: #222;
  padding: 15px;
  border-radius: 10px;
  width: calc(25% - 20px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease;
}

.card:hover {
  transform: translateY(-5px);
}

.icon {
  background-color: #333;
  color: #ffffff;
  padding: 10px;
  border-radius: 50%;
  font-size: 18px;
  margin-right: 15px;
}

.data {
  flex-grow: 1;
}

.value {
  font-size: 18px;
  font-weight: bold;
  color: #ffffff;
  margin-bottom: 5px;
}

.label {
  font-size: 12px;
  text-transform: uppercase;
  color: #aaa;
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

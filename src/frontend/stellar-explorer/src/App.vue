<template>
  <div id="app">
    <Header />
    <SearchBar @search="handleSearch" />
    <MainTitle />
    <StatisticsCards />
    <BlockchainData />
    <Charts />
    <LatestLedgers />
    <!-- Other components go here -->
  </div>
</template>

<script>
import Header from './components/Header.vue'
import SearchBar from './components/SearchBar.vue'
import MainTitle from './components/MainTitle.vue'
import StatisticsCards from './components/StatisticsCards.vue'
import BlockchainData from './components/BlockchainData.vue'
import Charts from './components/Charts.vue'
import LatestLedgers from './components/LatestLedgers.vue'

export default {
  components: {
    Header,
    SearchBar,
    MainTitle,
    StatisticsCards,
    BlockchainData,
    Charts,
    LatestLedgers,
  },
  methods: {
    handleSearch(query) {
      console.log('User searched for:', query)
      // Implement your search logic here
    },
  },
}
</script>

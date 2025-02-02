<template>
  <div id="app">
    <Header />
    <SearchBar @search="handleSearch" />
    <MainTitle />
    <StatisticsCards />
    <BlockchainData />
    <Charts />
    <LatestLedgers />
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

<style>
/* Add Font Awesome CDN link */
/* @import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css'); */

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #ffffff;
  background-color: #1e1e1e;
  min-height: 100vh;
  padding: 20px;
  box-sizing: border-box;
}
</style>

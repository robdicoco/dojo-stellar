<template>
  <div id="app">
    <Header />
    <SearchBar @search="handleSearch" />
    <Results :result="result" />
    <!-- <div id="results"></div> -->
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
import Results from './components/Results.vue'

export default {
  components: {
    Header,
    SearchBar,
    MainTitle,
    StatisticsCards,
    BlockchainData,
    Charts,
    LatestLedgers,
    Results,
  },
  data() {
    return {
      result: null,
    }
  },
  methods: {
    async handleSearch(query) {
      console.log('User searched for:', query)
      this.performSearch(query)
    },

    async performSearch(query) {
      // const query = document.getElementById('searchQuery').value
      if (!query) {
        alert('Please enter a query.')
        return
      }

      const apiUrl = 'api/search'
      const data = { query }

      try {
        const response = await fetch(apiUrl, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(data),
        })

        const result = await response.json()
        this.result = result

        // if (!response.ok) {
        //   throw new Error(`HTTP error! status: ${response.status}`)
        // }
      } catch (error) {
        console.error('Error:', error)
        this.result = { detail: 'An error occurred while fetching the data.' }
      }
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

<template>
  <header>
    <img alt="Lumen logo" class="logo" src="@/assets/logo2.png" width="125" height="125" />
  </header>

  <div id="app">
    <BlockSearch />
    <TransactionSearch />
    <BalanceSearch />
  </div>
</template>

<script>
import BlockSearch from './components/BlockSearch.vue'
import TransactionSearch from './components/TransactionSearch.vue'
import BalanceSearch from './components/BalanceSearch.vue'

export default {
  name: 'App',
  components: {
    BlockSearch,
    TransactionSearch,
    BalanceSearch,
  },
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>

<template>
  <div class="search-bar">
    <div class="icon" @click="handleSearch">
      <i class="fas fa-search"></i>
    </div>
    <input
      type="text"
      v-model="searchQuery"
      :placeholder="placeholderText"
      @keyup.enter="handleSearch"
    />
  </div>
</template>

<script>
export default {
  name: 'SearchBar',
  data() {
    return {
      searchQuery: '', // Stores the current search input
      placeholderText:
        'Search for labels, ledgers, contracts, transactions, operations, accounts, assets...', // Placeholder text
    }
  },
  methods: {
    handleSearch() {
      // Emit the search query to the parent component only if the query is not empty
      if (this.searchQuery.trim()) {
        this.$emit('search', this.searchQuery)
      }
    },
  },
}
</script>

<style scoped>
.search-bar {
  display: flex;
  align-items: center;
  background-color: #222;
  border-radius: 20px;
  padding: 5px 15px;
  width: 100%;
  max-width: 600px;
  margin: 20px auto;
}

.icon {
  color: #aaa;
  font-size: 18px;
  margin-right: 10px;
}

input {
  background: none;
  border: none;
  color: #ffffff;
  font-size: 14px;
  width: 100%;
  outline: none;
}

input::placeholder {
  color: #aaa;
}

/* Hover and Focus Effects */
.search-bar:hover .icon,
input:focus + .icon {
  color: #00bcd4; /* Highlight color on hover or focus */
}
</style>

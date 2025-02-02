<template>
  <div class="search-bar">
    <div class="icon">
      <i class="fas fa-search"></i>
    </div>
    <input type="text" v-model="searchQuery" :placeholder="placeholderText" @input="handleSearch" />
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
      // Emit the search query to the parent component
      this.$emit('search', this.searchQuery)
    },
  },
}
</script>

<style scoped>
/* General Search Bar Styles */
.search-bar {
  display: flex;
  align-items: center;
  background-color: #222; /* Dark theme background */
  border-radius: 20px;
  padding: 5px 15px;
  width: 100%;
  max-width: 600px; /* Limit the width for better UI */
  margin: 20px auto; /* Center the search bar */
}

/* Icon Styles */
.icon {
  color: #aaa; /* Light gray color for the icon */
  font-size: 18px;
  margin-right: 10px;
}

/* Input Field Styles */
input {
  background: none;
  border: none;
  color: #ffffff; /* White text color */
  font-size: 14px;
  width: 100%;
  outline: none;
}

input::placeholder {
  color: #aaa; /* Light gray placeholder text */
}

/* Hover and Focus Effects */
.search-bar:hover .icon,
input:focus + .icon {
  color: #00bcd4; /* Highlight color on hover or focus */
}
</style>

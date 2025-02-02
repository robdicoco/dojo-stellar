<template>
  <header class="header">
    <!-- Left Section: Logo and Navigation Links -->
    <div class="header-left">
      <div class="logo">
        <span>Stellar Lumen League | Explorer</span>
      </div>
      <nav class="navigation">
        <ul>
          <li><a href="#">Blockchain</a></li>
          <li><a href="#">Accounts</a></li>
          <li><a href="#">Market</a></li>
          <li><a href="#">Assets</a></li>
          <li><a href="#">Charts</a></li>
        </ul>
      </nav>
    </div>

    <!-- Right Section: Favorites, Network Selection, and Settings -->
    <div class="header-right">
      <button class="favorites-btn">
        <i class="fas fa-heart"></i>
      </button>
      <div class="network-selector">
        <select v-model="selectedNetwork">
          <option value="LIVENET">LIVENET</option>
          <option value="TESTNET">TESTNET</option>
        </select>
      </div>
      <button class="settings-btn">
        <i class="fas fa-cog"></i>
      </button>
    </div>
  </header>
</template>

<script>
export default {
  name: 'Header',
  data() {
    return {
      selectedNetwork: 'LOCAL', // Default selected network
    }
  },
}
</script>

<style scoped>
/* General Header Styles */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #1e1e1e; /* Dark theme background */
  color: #ffffff; /* Text color */
  padding: 10px 20px;
}

/* Left Section: Logo and Navigation */
.header-left {
  display: flex;
  align-items: center;
}

.logo {
  font-size: 18px;
  font-weight: bold;
  margin-right: 20px;
}

.navigation ul {
  display: flex;
  list-style: none;
  margin: 0;
  padding: 0;
}

.navigation li {
  margin-right: 15px;
}

.navigation a {
  color: #ffffff;
  text-decoration: none;
  font-size: 14px;
  transition: color 0.3s ease;
}

.navigation a:hover {
  color: #00bcd4; /* Highlight color on hover */
}

/* Right Section: Icons and Network Selector */
.header-right {
  display: flex;
  align-items: center;
}

.favorites-btn,
.settings-btn {
  background: none;
  border: none;
  color: #ffffff;
  font-size: 18px;
  cursor: pointer;
  margin-left: 10px;
  transition: color 0.3s ease;
}

.favorites-btn:hover,
.settings-btn:hover {
  color: #00bcd4; /* Highlight color on hover */
}

.network-selector select {
  background-color: #333333;
  color: #ffffff;
  border: none;
  padding: 5px 10px;
  border-radius: 5px;
  font-size: 14px;
  cursor: pointer;
  outline: none;
}

.network-selector select:focus {
  box-shadow: 0 0 5px rgba(0, 188, 212, 0.5);
}
</style>

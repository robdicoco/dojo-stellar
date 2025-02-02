<template>
  <div class="main-title">
    <h1 class="title">{{ title }}</h1>
    <p class="subtitle">{{ subtitle }}</p>
  </div>
</template>

<script>
export default {
  name: 'MainTitle',
  data() {
    return {
      title: 'StellarChain | Lumen Explorer', // Main title
      subtitle: 'StellarChain Explorer: Your Lumen Blockchain Discovery Tool', // Subtitle
    }
  },
}
</script>

<style scoped>
/* General Main Title Styles */
.main-title {
  text-align: center;
  margin: 20px 0; /* Add some spacing around the title */
}

/* Title Styles */
.title {
  font-size: 36px;
  font-weight: bold;
  color: #ffffff; /* White text color */
  margin-bottom: 10px; /* Space between title and subtitle */
}

/* Subtitle Styles */
.subtitle {
  font-size: 18px;
  color: #aaa; /* Light gray text color */
  font-style: italic; /* Italicize the subtitle for emphasis */
}
</style>

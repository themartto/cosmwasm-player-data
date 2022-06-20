<template>
  <v-app>
    <v-main>
      <demo></demo>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from 'vue';
import Demo from './components/Demo.vue';

export default Vue.extend({
  name: 'App',

  components: {
    Demo,
  },

  data: () => ({
    //
  }),
});
</script>

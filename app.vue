<template>
  <div class="container">
      <header class="row">
          <div class="col">
              <h1>Time till...</h1>
          </div>
          <div class="col d-none d-md-initial">
              <h1>{{nextConference.name}}</h1>
          </div>
          <div class="col d-none d-md-initial">
              <time-slice :time="new Date(nextConference.date)"></time-slice>
          </div>
      </header>

      <section class="row">
          <div class="col-12 col-md-6 order-last order-md-first">
              <h2>About timetill.rs</h2>
              <p>Elit reiciendis placeat deleniti voluptates tenetur Eos soluta libero dolorem nemo eos Sequi sunt iste commodi facere aliquam Provident numquam eum optio necessitatibus sit minima Maxime sapiente neque cum ea</p>
              <p>Sit quae consectetur sunt quis architecto illum. Autem quod eaque itaque vitae nesciunt? Harum commodi deleniti necessitatibus quia nam? In modi mollitia illum facilis dolore! Laudantium officia distinctio necessitatibus tempore!</p>

              <a href="#" class="btn">GitHub</a>
          </div>

          <div v-for="conference in conferences" class="col-12 col-md-6">
              <div class="row">
                  <div class="col-12">
                      <h2>{{conference.name}}</h2>
                  </div>
                  <div class="col-12">
                      <time-slice :time="new Date(conference.date)"></time-slice>
                  </div>
                  <div class="col-12">
                      <p><em>&ldquo;{{conference.blurb}}</em>&rdquo;</p>
                  </div>
                  <div class="col-12">
                      <dl class="row">
                          <template v-for="(feature, name) in conference.features">
                              <dt class="col-6 text-capitalize">{{name}}</dt>
                              <dd class="col-6 text-right">{{feature}}</dd>
                          </template>
                      </dl>
                  </div>
                  <div class="col-12">
                      <a :href="conference.schedule" target="_blank" class="btn">schedule</a>
                      <a :href="conference.website" target="_blank" class="btn">website</a>
                  </div>
              </div>
          </div>
      </section>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import timeSlice from './time.vue';
import data from './data.json';

let conferences = data.filter(conference => {
    const confDate = new Date(conference.date)
    const today = new Date()

    return confDate > today
})

export default Vue.extend({
    components: {
        timeSlice
    }
    data() {
        return {
            nextConference: conferences[0]
            conferences: conferences
        };
    }
});
</script>

<style lang="scss" scoped>
</style>

<template>
  <div class="row">
    <div class="col-12" v-if="!Object.entries(regions).length">
      <p>No Events here yet <em>unfrownately</em> :(</p>
    </div>
    <div
      class="col-12 region"
      v-if="regions"
      v-for="(cities, region) in regions"
      :key="region"
    >
      <h1>{{ region }}</h1>
      <div class="h-ruler mt-0 w-100"></div>

      <div class="row">
        <div
          class="col-12 col-md-4 city"
          v-for="(events, city) in cities"
          :key="city"
        >
          <h2>{{ city }}</h2>
          <div class="h-ruler mt-0 w-100"></div>

          <div class="row">
            <div class="col-12 event" v-for="event in events" :key="event.name">
              <h3>
                <router-link
                  :to="{ name: 'eventDetail', params: { slug: event.slug } }"
                >
                  {{ event.name }}</router-link
                >
              </h3>
              <h6>{{ event.date | date }}</h6>

              <address>{{ event.address }}</address>
              <router-link
                class="text-uppercase btn btn-primary"
                :to="{ name: 'eventDetail', params: { slug: event.slug } }"
              >
                details</router-link
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  computed: {
    eventLink(slug) {
      return `/events/${slug}`;
    }
  },
  data() {
    fetch(`http://${window.location.hostname}:5000/events`)
      .then(r => r.json())
      .then(json => {
        this.regions = json.events.reduce((regions, event) => {
          if (!regions[event.region]) regions[event.region] = {};

          // If the event is in the United States we try to divide by state
          // instead of city.
          let subcategory =
            event.country.toLowerCase() === "united states"
              ? event.state
              : event.city;
          subcategory = `${subcategory}, ${event.country}`;

          if (!regions[event.region][subcategory])
            regions[event.region][subcategory] = [];

          regions[event.region][subcategory].push(event);

          return regions;
        }, {});
      });

    return {
      regions: {}
    };
  }
};
</script>

<style lang="scss" scoped>
.region {
  &:not(:first-of-type) {
    margin-top: 2rem;
  }

  &:last-of-type {
    margin-bottom: 2rem;
  }
}

.h-ruler {
  margin-bottom: 1rem;
}

h3 a {
  color: inherit;
}
</style>

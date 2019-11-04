<template v-if="event">
  <div class="row">
    <div class="col-12 col-md-7">
      <div class="row">
        <div class="col-12 col-md">
          <h1>{{ event.event }}</h1>
          <h5 class="d-none d-md-block">{{ event.event_date | date }}</h5>
        </div>
        <div class="col-12 col-md text-center text-md-right" v-if="isInFuture">
          <time-slice :prefix="absoluteDuration" :event="event"></time-slice>
        </div>
      </div>
      <dl>
        <template v-if="event.recurring">
          <dt>Occurance</dt>
          <dd>Every {{ event.occurance[0] }} {{ event.occurance[1] }}</dd>
        </template>
        <dt>Organisers</dt>
        <dd>
          <ul
            class="list-unstyled"
            v-for="organiser in event.organisers"
            :key="organiser.id"
          >
            <li class="list-unstyled-item">
              <inline-user :user="organiser" link-github></inline-user>
            </li>
          </ul>
        </dd>
        <dt>Attendees</dt>
        <dd>
          <ul
            class="list-unstyled"
            v-for="attendee in event.attendees"
            :key="attendee.id"
          >
            <li class="list-unstyled-item">
              <inline-user :user="attendee" link-github></inline-user>
            </li>
          </ul>
        </dd>
      </dl>

      <div v-if="isInFuture">
        <a class="btn btn-primary" :href="attendLink">Attend with GitHub</a>
        <a class="btn" href=""></a>
      </div>
    </div>
    <div class="col-12 col-md-5" v-if="event.gps">
      <l-map id="event-map" :center="event.gps" :zoom="15" :options="options">
        <l-marker :lat-lng="event.gps"></l-marker>
        <l-tile-layer :url="url"></l-tile-layer>
      </l-map>
    </div>
    <div class="col-12">
      <h2>About {{ event.name }}</h2>
    </div>
    <div class="col-12 col-md about" v-html="event.about_html"></div>
    <div class="col-12 col-md-5" v-if="event.picture">
      <img :src="'/' + event.picture" class="img-fluid" />
    </div>
  </div>
</template>

<script lang="ts">
import moment from "moment";
import { LMap, LMarker, LTileLayer } from "vue2-leaflet";

export default {
  components: {
    LMap,
    LMarker,
    LTileLayer
  },

  computed: {
    absoluteDuration() {
      const start = moment(this.event.event_date);
      const now = moment();

      while (this.event.recurring && start.isBefore(now)) {
        start.add(this.event.occurance_amount, this.event.occurance_step);
      }

      const end = start
        .clone()
        .add(this.event.duration_amount, this.event.duration_step);

      return `${start.format("h:mm")}–${end.format("h:mma")}`;
    },

    attendLink(): string {
      return `http://localhost:5000/attend?next_page=${
        window.location.pathname
      }`;
    },

    isInFuture(): boolean {
      return moment(this.event.event_date).isAfter();
    }
  },

  methods: {
    squareMap() {
      return document.querySelector("#event-map").offsetWidth;
    }
  },

  data() {
    fetch(`http://${window.location.hostname}:5000/events/${this.slug}`)
      .then(r => r.json())
      .then(event => {
        this.event = event;
      });

    return {
      url: "http://{s}.tile.osm.org/{z}/{x}/{y}.png",
      event: {},
      options: {
        zoomControl: window.innerWidth > 768
      }
    };
  },

  mounted() {},

  props: {
    slug: String
  }
};
</script>

<style lang="scss" scoped>
#event-map {
  padding-top: 100%;
}
</style>

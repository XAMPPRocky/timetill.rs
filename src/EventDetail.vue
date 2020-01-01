<template>
  <div v-if="event">
    <div class="row">
      <div class="col">
        <div
          class="col-12 alert alert-primary"
          role="alert"
          v-if="!event.approved"
        >
          <div class="row">
            <div class="col align-middle d-flex">{{ reviewText }}</div>
            <div class="col d-inline-block text-right">
              <button class="btn btn-primary" @click="approveEvent">
                Approve
              </button>
              <button class="btn btn-danger" @click="denyEvent">Deny</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="row">
      <div class="col-12 col-md-7">
        <div class="row">
          <div class="col-12 col-md">
            <h1>{{ event.event }}</h1>
            <h5 class="d-none d-md-block">{{ event.event_date | date }}</h5>
          </div>
          <div
            class="col-12 col-md text-center text-md-right"
            v-if="isInFuture"
          >
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
          <dd v-if="event.attendees.length > 0">
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
          <dd v-else>
            <p>
              No attendees yet<span v-if="notEventOrganiser"
                >, interested in becoming
                <a :href="attendLink">the first one?</a>
              </span>
            </p>
          </dd>
        </dl>

        <div v-if="isInFuture && notEventOrganiser">
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
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import moment from "moment";
import { LMap, LMarker, LTileLayer } from "vue2-leaflet";
import axios from "axios";

@Component({
  components: {
    LMap,
    LMarker,
    LTileLayer
  }
})
export default class EventDetail extends Vue {
  @Prop() inputEvent: object | undefined;
  @Prop() slug: string;
  @Prop() user: object;
  event: object | null = null;
  url: string = "http://{s}.tile.osm.org/{z}/{x}/{y}.png";
  options: object = { zoomControl: window.innerWidth > 768 };
  reviewText: string = "This event is currently under review";
  pending: boolean = false;

  created() {
    if (!this.inputEvent) {
      axios
        .get(this.routeUrl)
        .then(r => {
          this.event = r.data;
        })
        .catch(e => {
          this.$router.push({ name: "notFound", params: {} });
        });
    } else {
      this.event = this.inputEvent;
    }
  }

  get routeUrl() {
    return `/events/${this.slug}`;
  }

  get absoluteDuration() {
    const start = moment(this.event.event_date);
    const now = moment();

    while (this.event.recurring && start.isBefore(now)) {
      start.add(this.event.occurance_amount, this.event.occurance_step);
    }

    const end = start
      .clone()
      .add(this.event.duration_amount, this.event.duration_step);

    return `${start.format("h:mm")}–${end.format("h:mma")}`;
  }

  get notEventOrganiser(): boolean {
    if (this.$root.$data.user) {
      return !this.event.organisers.some(o => o.id == this.$root.$data.user.id);
    } else {
      return true;
    }
  }

  get attendLink(): string {
    return `http://localhost:5000/attend?next_page=${window.location.pathname}`;
  }

  get isInFuture(): boolean {
    return moment(this.event.event_date).isAfter();
  }

  approveEvent(event) {
    if (!this.pending) {
      this.pending = true;
      axios.get(`${this.routeUrl}/approve`).then(() => {
        this.reviewText = `Event Approved!`;
        this.pending = false;
        event.target.parentElement.hidden = true;
      });
    }
  }

  denyEvent(event) {
    if (!this.pending) {
      this.pending = true;
      axios.get(`${this.routeUrl}/deny`).then(() => {
        this.reviewText = `Event Denied!`;
        this.pending = false;
        event.target.parentElement.hidden = true;
      });
    }
  }

  squareMap(): number {
    return document.querySelector("#event-map").offsetWidth;
  }
}
</script>

<style lang="scss" scoped>
#event-map {
  padding-top: 100%;
}
</style>

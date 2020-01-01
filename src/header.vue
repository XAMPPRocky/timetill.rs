<template>
  <header class="row sticky-top align-items-center">
    <nav class="col-12 text-center">
      <ul class="list-unstyled row justify-content-around">
        <li class="col">
          <router-link to="/">Conferences</router-link>
        </li>

        <li class="col">
          <router-link to="/events">Events</router-link>
        </li>

        <li class="col">
          <button @click="toggleDisclosure" v-if="$root.$data.user">
            <inline-user :user="$root.$data.user"></inline-user>
            <span class="disclosure-icon d-inline-block" :class="{ expand }"
              >▼</span
            >
          </button>
          <a v-else :href="loginLink()">Login with GitHub</a>
        </li>
      </ul>
    </nav>

    <div class="col-12 text-center disclosure" v-if="expand">
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <ul class="text-left d-inline-block">
            <li v-if="$root.$data.isReviewer()">
              <router-link :to="headerLink('/review-queue')"
                >Review Events</router-link
              >
            </li>
            <li>
              <router-link :to="headerLink('/my-events')"
                >My Events</router-link
              >
            </li>
            <li>
              <router-link :to="headerLink('/new-event')"
                >Create New Event</router-link
              >
            </li>
          </ul>
        </div>
        <div class="col">
          <ul class="text-left d-inline-block">
            <li><button @click="logOut">Sign Out</button></li>
          </ul>
        </div>
      </div>
    </div>

    <div class="content col-12">
      <div class="row">
        <div class="col text-center">
          <h1>Time till...</h1>
        </div>
        <div class="col d-none d-md-block">
          <h1 class="conf-name text-center">
            <a :href="event.website" target="_blank">
              {{ event.name }}
            </a>
          </h1>
        </div>
        <div class="col d-none d-md-block">
          <time-slice class="text-center" :event="event"></time-slice>
        </div>
      </div>
    </div>
  </header>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import moment from "moment";
import { LMap, LMarker, LTileLayer } from "vue2-leaflet";
import axios from "axios";

@Component()
export default class Header extends Vue {
  @Prop() event: object;
  expand: boolean = false;

  loginLink() {
    return `${axios.defaults.baseURL}/login?next_page=${
      window.location.pathname
    }`;
  }

  headerLink(path) {
    return { path };
  }

  toggleDisclosure() {
    this.expand = !this.expand;
  }

  logOut() {
    window.localStorage.clear();
    this.$router.go();
  }
}
</script>

<style lang="scss" scoped>
header {
  position: -webkit-sticky;
  position: sticky;
  margin: 2rem 0;
  padding-top: 1rem;
  background-color: white;

  a {
    color: inherit;
    text-decoration: underline;
  }
}

.content {
  border-bottom: 1px solid black;
  border-top: 1px solid black;
  box-shadow: 0px 10px 17px -11px rgba(0, 0, 0, 0.25);
  padding: 1rem 0;

  a {
    text-decoration: underline;
  }
}

.disclosure-icon {
  line-height: 0.1;
  transition: transform 0.25s;

  &.expand {
    transform: rotate(180deg);
  }
}

.disclosure {
  button {
    text-decoration: underline;
  }
}

button {
  border: none;
  background-color: transparent;
  padding: 0;

  &:active {
    color: inherit;
  }
}
</style>

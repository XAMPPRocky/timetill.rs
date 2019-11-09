import Router from 'vue-router'
import Vue from 'vue'
import moment from 'moment'
import { Icon } from 'leaflet'
import Component from 'vue-class-component'
import axios from 'axios'

import App from './App.vue'
import Consts from './consts.ts'
import EventDetail from './EventDetail.vue'
import EventList from './EventList.vue'
import Home from './home.vue'
import InlineUser from './InlineUser.vue'
import TimeSlice from './TimeSlice.vue'
import NotFound from './404.vue'

// Conference data.
import ConferenceData from './ConferenceData.ts'
const conferences = ConferenceData.getInstance().data()

// Leaflet integration
delete Icon.Default.prototype._getIconUrl
Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png')
})

// Register the router hooks with their names
Component.registerHooks([
  'beforeRouteEnter',
  'beforeRouteLeave',
  'beforeRouteUpdate' // for vue-router 2.2+
])

const routes = [
  {
    path: '/',
    component: App,
    props: { conference: conferences[0] },
    children: [
      {
        path: '',
        component: Home,
        name: 'home',
        props: { conferences }
      },

      {
        path: 'authorise',
        redirect (to) {
          window.localStorage.setItem(Consts.ACCESS_TOKEN_KEY, to.query.access_token)
          return { path: to.query.next_page, query: null }
        }
      },

      {
        path: 'events/:slug',
        component: EventDetail,
        name: 'eventDetail',
        props: true
      },

      {
        path: 'events',
        name: 'eventList',
        component: EventList
      },

      {
        path: '*',
        name: 'notFound',
        component: NotFound,
      },
    ],
  },
]

const router = new Router({ routes, mode: 'history' })
Vue.use(Router)

Vue.component('time-slice', TimeSlice)
Vue.component('inline-user', InlineUser)
Vue.filter('date', (date) => moment(date).format('dddd, Do of MMMM YYYY'))

axios.defaults.baseURL = 'http://localhost:5000'

const data = {
  currentUser: null,

  get user() {
    if (!this.currentUser) {
      this.fetchUser()
    }

    return this.currentUser
  },

  isReviewer() {
    return this.currentUser.model.reviewer || false
  },

  fetchUser() {
    axios.get('/user').then(r => {
      if (r.status === 304) {
        return this.currentUser
      } else if (r.status === 200) {
        localStorage.setItem(Consts.USER_KEY, JSON.stringify(user));
        this.currentUser = user;
      } else {
      }
    })
      .catch(e => {});
  }
};

/* eslint-disable no-new */
new Vue({
  el: '#app',
  data,
  router,

  created() {
    try {
      const token = localStorage.getItem(Consts.ACCESS_TOKEN_KEY)
      const etag = localStorage.getItem(Consts.USER_ETAG_KEY)
      this.currentUser = JSON.parse(localStorage.getItem(Consts.USER_KEY))
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`
    } catch (e) {}
  },

})

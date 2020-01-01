<template>
  <div class="row">
    <div class="col col-md-8 offset-md-2">
      <h1 class="text-capitalize">new event</h1>

      <form @submit.prevent>
        <h4>Event Details</h4>

        <div class="form-group">
          <label for="name">Name</label>
          <input
            name="name"
            type="text"
            v-model="name"
            class="form-control"
            placeholder="Dublin Lock-out"
            required
          />
        </div>

        <div class="form-group">
          <label for="slug">URL</label>
          <small class="form-text">
            Must be letters, numbers, or '-' hyphens between them.
          </small>
          <div class="input-group">
            <div class="input-group-prepend">
              <span class="input-group-text">/events/</span>
            </div>
            <input
              name="slug"
              type="text"
              v-model="slug"
              class="form-control"
              placeholder="dublin-lock-out"
              required
            />
          </div>
        </div>

        <div class="form-group">
          <label for="about_md">About</label>
          <small class="form-text"
            >Include a small paragraph about your event. (Supports
            Markdown)</small
          >
          <textarea
            class="form-control"
            name="about_md"
            rows="10"
            required
            v-model="about_md"
            placeholder="A small gathering of like minded folks..."
          ></textarea>
        </div>

        <div class="form-group">
          <label for="event-date">Date</label>
          <date-picker
            placeholder="26 Aug 1913"
            name="event-date"
            input-class="form-control date-input"
            v-model="eventDate"
            required
          ></date-picker>
        </div>

        <div class="form-group">
          <label for="duration">Duration</label>
          <br />
          <input
            class="form-control shrink-to-fit"
            type="number"
            value="1"
            min="1"
            max="24"
            v-model.number="duration"
          />
          <span>{{ durationText }}</span>
        </div>

        <div class="form-group">
          <input v-model="recurring" name="recurring" type="checkbox" />
          <label for="recurring">Does your event happen regularly?</label>
        </div>

        <div class="form-group" v-if="recurring">
          <label for="occurance_amount">Occurance</label>
          <br />
          Every
          <input
            class="form-control shrink-to-fit"
            type="number"
            value="1"
            min="1"
            max="24"
            v-model.number="occurance_amount"
          />

          <select name="occurance_step">
            <option v-for="step in occuranceSteps" value="step.value">
              <template v-if="occurance_amount > 1">{{ step.value }}</template>
              <template v-else>{{ step.singular }}</template>
            </option>
          </select>
        </div>

        <h4>Location</h4>
        <div class="form-group">
          <label for="address">Street Address</label>
          <input
            name="address"
            type="text"
            class="form-control"
            placeholder="Sackville Street"
            v-model="address"
            required
          />
        </div>

        <div class="form-group">
          <label for="city">City</label>
          <input
            name="city"
            type="text"
            class="form-control"
            placeholder="Dublin"
            v-model="city"
            required
          />
        </div>

        <div class="form-group">
          <label for="country">Country</label>
          <input
            name="country"
            type="text"
            class="form-control"
            placeholder="Ireland"
            v-model="country"
            required
          />
        </div>

        <div class="form-group">
          <label for="region">Region</label>
          <select
            name="region"
            type="text"
            class="form-control"
            v-model="region"
            required
          >
            <option value="" disabled selected
              >Please select your region</option
            >
            <option v-for="region in regions" v-bind:value="region.value">
              {{ region.display }}
            </option>
          </select>
        </div>

        <div class="form-group">
          <label for="gps">GPS Coordinates — <em>Optional</em></label>
          <small class="form-text">
            Must be in <code>lat,long</code> format. Enables OpenStreetMap
            integeration if provided.
          </small>
          <input
            name="gps"
            type="text"
            class="form-control w-50"
            placeholder="53.351110,-6.260960"
          />
        </div>

        <div class="form-group">
          <p class="form-text">I agree to the terms, blah blash</p>
          <button @click="submit" class="btn btn-primary">
            Create Event
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import axios from "axios";
import DatePicker from "vuejs-datepicker";

@Component({
  components: {
    DatePicker
  }
})
export default class EventForm extends Vue {
  about_md: string | null = null;
  address: string | null = null;
  city: string | null = null;
  country: string | null = null;
  duration: number = 1;
  isRecurring: boolean = false;
  name: string | null = null;
  occurance_amount: number | null = null;
  pending: boolean = false;
  region: string | null = null;
  slug: string | null = null;
  eventDate = null;

  regions: Array<object> = [
    { value: "Central Africa" display: "Africa (Central)" },
    { value: "Northern Africa" display: "Africa (North)" },
    { value: "Southern Africa" display: "Africa (South)" },
    { value: "Central America" display: "America (Central)" },
    { value: "North America" display: "America (North)" },
    { value: "South America" display: "America (South)" },
    { value: "Antarctica" display: "Antarctica" },
    { value: "Central Asia" display: "Asia (Central)" },
    { value: "Eastern Asia" display: "Asia (East)" },
    { value: "Western Asia" display: "Asia (West)" },
    { value: "Europe" display: "Europe" },
  ];

  occuranceSteps: Array<object> = [
    { value: 'weeks', singular: 'week' },
    { value: 'months', singular: 'month' },
    { value: 'years', singular: 'year' },
  ];

  get durationText() {
    return this.duration > 1 ? 'hours': 'hour'
  }

  set recurring(value: boolean) {
    if (value) {
      this.occurance_amount = 1
    } else {
      this.occurance_amount = null
    }

    this.isRecurring = value;
  }

  get recurring(): boolean {
    return this.isRecurring
  }

  mounted() {
    for (const input of document.querySelectorAll('.shrink-to-fit')) {
      const length = (input.placeholder.length || input.value.length) + 3;
      input.setAttribute('style', `width: ${length}rem`);
      console.log(input.attributes)
    }
  }

  submit() {
    if (this.pending) {
      return
    }

    this.pending = true;

    axios.post('/events', {
      about_md: this.about_md,
      address: this.address,
      city: this.city,
      country: this.country,
      event: this.name,
      event_duration_step: 'hours',
      event_duration_amount: this.duration,
      event_date: this.eventDate,
      gps: this.gps,
      occurance_step: this.occurance_step,
      occurance_amount: this.occurance_amount,
      region: this.region,
      slug: this.slug,
    })
      .then(r => {
        this.pending = false;
        console.log(r)
        alert('Success')
      })
      .catch(r => {
        debugger;
        console.log(r)
      })
  }
}
</script>

<style lang="scss">
.date-input[readonly] {
  background-color: transparent !important;
}

.shrink-to-fit {
  display: inline-block;
  width: auto;
}
</style>

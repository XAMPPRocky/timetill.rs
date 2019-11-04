import data from '../conferences.json'

const conferences = <Array<Conference>> data

interface Conference {
  name: string,
  date: string,
  blurb: string,
  location: string,
  website: string,
  schedule: string,
  features: object,
}

export default class ConferenceData {
  private static instance: ConferenceData;
  private conferences: Array<Conference>;

  private constructor () {
    this.conferences = conferences.filter(c => new Date(c.date) > new Date())
      // Sort by closest to furtherest from today.
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
  }

  static getInstance (): ConferenceData {
    if (!ConferenceData.instance) {
      ConferenceData.instance = new ConferenceData()
    }

    return ConferenceData.instance
  }

  data (): Object[] {
    return this.conferences
  }
}

// eslint-disable-file
export default {
    getQuery (name) {
      // eslint-disable-next-line
      name = name.replace(/[\[]/, '\\[').replace(/[\]]/, '\\]')
      const regex = new RegExp('[\\?&]' + name + '=([^&#]*)')
      const results = regex.exec(location.search)
      return results === null ? '' : decodeURIComponent(results[1].replace(/\+/g, ' '))
    }
  }
  
const Promise = require('bluebird')

const ExtensionManager = require('./ExtensionManager')
const ChannelManager = require('./ChannelManager')
const DeviceManager = require('./DeviceManager')
const HookManager = require('./HookManager')

const Action = require('@iotame/api').Action
const Filter = require('@iotame/api').Filter

module.exports = class Supervisor {
  constructor (logger, redis) {
    this.redis = redis
  }

  boot () {
    return new Promise((resolve, reject) => {
      // Generate a hook manager and receive its dispatcher function
      this.hooks = new HookManager(this)
      let dispatch = this.hooks.dispatcher()

      this.hooks.add(... this._channelHandlers())   // Add channel handler hooks
      this.hooks.add(... this._generalHooks())      // Add general hooks

      // Generate an extension manager and register it
      this.extensions = new ExtensionManager(this, dispatch)
      this.extensions.register()

      // Add extension hooks
      this.hooks.add(... this.extensions.hooks())

      // Generate a devices manager and boot it up
      this.devices = new DeviceManager(this, dispatch)
      this.devices.greet()

      // Generate a channel manager and open channels needed by our devices
      this.channels = new ChannelManager(this, dispatch)
      this.channels.open(this.devices.list())

      this.ready = true

      dispatch('iotame.supervisor.ready')
      resolve()
    })
  }

  stop () {
    // Tear down everything important
  }

  resolve (name) {
    return this.extensions.resolve(name)
  }

  _channelHandlers () {
    return []
  }

  _generalHooks () {
    return [
      (new Filter()).on('devicemanager.greeting')
    ]
  }
}

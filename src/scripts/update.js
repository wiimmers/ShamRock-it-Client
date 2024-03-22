const {
    checkUpdate,
    installUpdate,
    onUpdaterEvent,
  } = window.__TAURI__.updater


document.addEventListener('DOMContentLoaded', async function() {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
        // This will log all updater events, including status updates and errors.
        console.log('Updater event', error, status)
      })
      
      try {
        const { shouldUpdate, manifest } = await checkUpdate()
      
        if (shouldUpdate) {
          // You could show a dialog asking the user if they want to install the update here.
          console.log(
            `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
          )
      
          // Install the update. This will also restart the app on Windows!
          await installUpdate()
      
        }
      } catch (error) {
        console.error(error)
      }
      
      // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
      unlisten()
});
package dev.altisbacc.openidea

import android.os.Build
import android.os.Bundle
import android.system.Os
import android.view.WindowManager
import androidx.activity.enableEdgeToEdge
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    exportStorageRoots()
    enableEdgeToEdge()
    extendIntoCutout()
    super.onCreate(savedInstanceState)
    hideSystemBars()
  }

  override fun onWindowFocusChanged(hasFocus: Boolean) {
    super.onWindowFocusChanged(hasFocus)
    if (hasFocus) hideSystemBars()
  }

  private fun exportStorageRoots() {
    Os.setenv("OPENIDEA_PRIVATE_DIR", filesDir.absolutePath, true)
    val shared = getExternalFilesDir(null) ?: filesDir
    Os.setenv("OPENIDEA_SHARED_DIR", shared.absolutePath, true)
  }

  private fun extendIntoCutout() {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.P) return
    val attributes = window.attributes
    attributes.layoutInDisplayCutoutMode =
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
        WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS
      } else {
        WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
      }
    window.attributes = attributes
  }

  private fun hideSystemBars() {
    WindowCompat.setDecorFitsSystemWindows(window, false)
    val controller = WindowInsetsControllerCompat(window, window.decorView)
    controller.systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    controller.hide(WindowInsetsCompat.Type.systemBars())
  }
}

package name.jinleili.bevy

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.res.colorResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.viewinterop.AndroidView
import name.jinleili.bevy.ui.theme.MyApplicationTheme

class BevyActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        println("Creating BevyActivity")
        setContent {
            MyApplicationTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = colorResource(id = R.color.white)
                ) {
                    SurfaceCard()
                }
            }
        }
        println("BevyActivity Content Set")
    }

    var surfaceView: BevySurfaceView? = null

    @Composable
    fun SurfaceCard() {
        println("Surface Card of Bevy Activity")
        var selected by remember { mutableStateOf("AssBreakout") }
        val toggleValues = listOf("AssBreakout", "XXX",)
        val screenWidth = LocalConfiguration.current.screenWidthDp
        println("Let's create column")
        Column(modifier = Modifier.fillMaxSize()) {
            println("Creating column")
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center,
                modifier = Modifier
                    .height(44.dp)
                    .padding(horizontal = 0.dp, vertical = 7.dp)
                    .fillMaxWidth()
            ) {
                println("Creating Row")
                Text(text = "Bevy in Android App", fontSize = 20.sp, fontWeight = FontWeight.Bold)
            }
            Spacer(modifier = Modifier.height(8.dp))
            AndroidView(
                factory = { ctx ->
                    println("Creating Android View")
                    val sv = BevySurfaceView("bevy_in_ass", context = ctx)
                    surfaceView = sv
                    sv
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height((screenWidth.toFloat() * 1.6).dp),
            )
        }
        println("Finished Bevy Activity Surface Card")
    }
}
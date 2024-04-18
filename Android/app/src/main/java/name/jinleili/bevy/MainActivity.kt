package name.jinleili.bevy

import android.content.Intent
import android.os.Bundle
import android.view.View
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material.Button
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

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

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
    }

    public fun goToBevyGame() {
        var intent: Intent = Intent(this, BevyActivity::class.java).apply {  }
        println("Going to next game");
        startActivity(intent)
        println("Successful");
    }

    var surfaceView: BevySurfaceView? = null

    @Composable
    fun SurfaceCard() {
        var selected by remember { mutableStateOf("Breakout") }
        val toggleValues = listOf("Breakout", "XXX",)
        val screenWidth = LocalConfiguration.current.screenWidthDp
        Column(modifier = Modifier.fillMaxSize()) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center,
                modifier = Modifier
                    .height(44.dp)
                    .padding(horizontal = 0.dp, vertical = 7.dp)
                    .fillMaxWidth()
            ) {
                Text(text = "Bevy in Android App", fontSize = 20.sp, fontWeight = FontWeight.Bold)
                Button(onClick = { goToBevyGame() }) {
                    Text(text = "Next Game")
                }
            }
            Spacer(modifier = Modifier.height(8.dp))
            AndroidView(
                factory = { ctx ->
                    val sv = BevySurfaceView("bevy_in_app", context = ctx)
                    surfaceView = sv
                    sv
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height((screenWidth.toFloat() * 1.6).dp),
            )
        }
    }
}



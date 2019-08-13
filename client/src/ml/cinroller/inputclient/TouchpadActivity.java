package ml.cinroller.inputclient;

import java.net.DatagramSocket;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import android.app.Activity;
import android.os.AsyncTask;
import android.os.Bundle;
import android.util.Log;
import android.view.MotionEvent;
import android.view.View;
import android.view.View.OnTouchListener;

public class TouchpadActivity extends Activity implements OnTouchListener{
	protected UdpSender sender;
	
	@Override
	protected void onCreate(Bundle savedInstanceState){
		super.onCreate(savedInstanceState);
		this.setContentView(R.layout.activity_touchpad);

		View view = (View)findViewById(R.id.Touchpad);
		view.setOnTouchListener(this);
		
		try{
	        this.sender = new UdpSender(new DatagramSocket(),new InetSocketAddress(InetAddress.getByAddress(new byte[]{(byte)192,(byte)168,(byte)1,(byte)194}),4764));
        }catch(Exception e){
        	Log.e("SenderConstruct","Exception: " + e.getMessage());
            Log.e("SenderConstruct","Exception: " + e.toString());
        }
	}

	@Override
    public boolean onTouch(View view,MotionEvent event){
	    switch(event.getActionMasked()){
	    	case MotionEvent.ACTION_DOWN:
	    		try{
    	    		new SendData().execute(new TouchpadData(
    	    			TouchpadData.Type.PRESS,
    	    			event.getPressure(),
    	    			event.getSize(),
    	    			event.getAxisValue(MotionEvent.AXIS_X),
    	    			event.getAxisValue(MotionEvent.AXIS_Y)
    	    		)).get();
	    		}catch(Exception e){
	    			Log.e("SendData","Exception: " + e.getMessage());
	                Log.e("SendData","Exception: " + e.toString());
	    		}
	    		return true;
	    	case MotionEvent.ACTION_UP:
	    		try{
    	    		new SendData().execute(new TouchpadData(
    	    			TouchpadData.Type.RELEASE,
    	    			event.getPressure(),
    	    			event.getSize(),
    	    			event.getAxisValue(MotionEvent.AXIS_X),
    	    			event.getAxisValue(MotionEvent.AXIS_Y)
    	    		)).get();
	    		}catch(Exception e){
	    			Log.e("SendData","Exception: " + e.getMessage());
	                Log.e("SendData","Exception: " + e.toString());
	    		}
	    		return true;
	    	case MotionEvent.ACTION_MOVE:
	    		try{
    	    		new SendData().execute(new TouchpadData(
	    				TouchpadData.Type.MOVE,
    	    			event.getPressure(),
    	    			event.getSize(),
    	    			event.getAxisValue(MotionEvent.AXIS_X),
    	    			event.getAxisValue(MotionEvent.AXIS_Y)
    	    		)).get();
	    		}catch(Exception e){
	    			Log.e("SendData","Exception: " + e.getMessage());
	                Log.e("SendData","Exception: " + e.toString());
	    		}
	    		return true;
	    }
		view.performClick();
	    return false;
    }
	
	class SendData extends AsyncTask<TouchpadData,Void,Void>{
        @Override
        protected Void doInBackground(TouchpadData... arguments){
            sender.send(arguments[0]);
			return null;
        }
    }
}

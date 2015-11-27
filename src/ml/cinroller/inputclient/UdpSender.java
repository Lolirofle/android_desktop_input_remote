package ml.cinroller.inputclient;

import java.io.IOException;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import android.util.Log;

public class UdpSender{
    public DatagramSocket socket;
    public InetSocketAddress address;

    public UdpSender(DatagramSocket socket,InetSocketAddress address){
        this.address = address;
        this.socket = socket;
    }
    
    public boolean send(TouchpadData data){
    	try{
        	final int LEN = 4*2;
        	ByteBuffer bytes= ByteBuffer.allocate(LEN);
        	bytes.order(ByteOrder.LITTLE_ENDIAN);
        	
        	bytes.putInt(data.x);
        	bytes.putInt(data.y);
        	
        	DatagramPacket packet = new DatagramPacket(bytes.array(),LEN,this.address);
        	this.socket.send(packet);
        	return true;
    	}catch(IOException e){
    		Log.e("UdpSender","Exception: " + e.getMessage());
            Log.e("UdpSender","Exception: " + e.toString());
    		return false;
    	}
    }
}
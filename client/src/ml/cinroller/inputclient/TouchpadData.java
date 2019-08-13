package ml.cinroller.inputclient;

public class TouchpadData{
	public Type type;
	public float pressure;
	public float size;
	public float x;
	public float y;
	
	public TouchpadData(Type type,float pressure,float size,float x,float y){
		this.type = type;
		this.pressure = pressure;
		this.size= size;
		this.x = x;
		this.y = y;
	}
	
	public static enum Type{
		PRESS(1),
		RELEASE(2),
		MOVE(3);
		
		public int n;
		
		Type(int n){
			this.n = n;
		}
	}
}

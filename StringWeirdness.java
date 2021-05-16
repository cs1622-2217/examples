public class StringWeirdness {
	public static void main(String[] args) {
		// these strings look the same...
		String s = "tést";
		String t = "tést";
		System.out.println(s.length());  // 4
		System.out.println(s.charAt(1)); // é
		System.out.println(t.length());  // 5??
		System.out.println(t.charAt(1)); // e??
	}
}
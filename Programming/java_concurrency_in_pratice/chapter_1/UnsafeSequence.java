import net.jcip.annotations.*;

/**
 * UnsafeSequence
 *
 * @author Brian Goetz and Tim Peierls
 */

@NotThreadSafe
public class UnsafeSequence {
    private int value;

    /**
     * Returns a unique value.
     */
    public int getNext() {
        return value++;
    }

    public static void main(String[] args) {
        UnsafeSequence unsafe = new UnsafeSequence();
        unsafe.getNext();
    }
}

impl<T> LinkedList<T> {
    pub fn reverse(&mut self) {
        let mut current = self.start;
        let mut prev_end = self.end;

        while let Some(mut node_ptr) = current {
            unsafe {
                let node = node_ptr.as_mut();
                // Swap next and prev pointers
                std::mem::swap(&mut node.next, &mut node.prev);

                // Move to what used to be the 'next' node (which is now the 'prev')
                current = node.prev;
            }
        }

        // Swap the start and end pointers of the linked list
        std::mem::swap(&mut self.start, &mut self.end);
    }
}

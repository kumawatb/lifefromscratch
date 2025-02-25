use ahash::AHashMap;
use crate::core::atom::Spatial2D;

/// This struct is essentially a container for the atoms with its own spatial hashing implementation
pub struct Grid<T>{

    /// mapvec is the actual container implemented as a vector of <u32, T> hashmaps
    /// The index in the vector is called a "grid_id" or "gid"
    mapvec: Vec<AHashMap<u32,T>>,

    /// This hashmap maps the object ids to grid_id
    idlookup: AHashMap<u32, usize>,

    /// Size of grid cells
    spacing: (f32, f32),

    /// Number of x, y cells in the grid
    span: (usize, usize),

}


impl<T> Grid<T>{
    // *********** PUBLIC ***********

    /// Create a new grid with given size and spacing
    pub fn new(size_x: f32, size_y: f32, spacing_x: f32, spacing_y: f32) -> Grid<T> {
        let span_x = (size_x/spacing_x) as usize;
        let span_y = (size_y/spacing_y) as usize;

        let mut container: Vec<AHashMap<u32, T>> = Vec::with_capacity(span_x * span_y);

        for _ in 0..(span_x*span_y){
            container.push(AHashMap::new());
        }

        Grid{mapvec: container, idlookup: AHashMap::new(), spacing: (spacing_x, spacing_y), span: ( span_x, span_y )}
    }

    // Remove an object from the Grid, returning it, along with its position
    pub fn remove(&mut self, objid: u32) -> T {
        let obj = self.mapvec[*self.idlookup.get(&objid).unwrap()].remove(&objid).unwrap();
        self.idlookup.remove(&objid);

        return obj
    }
    
    /// Return a vector of references to all the atoms
    pub fn iter(&self) -> Vec<&T> {
        self.mapvec.iter().flat_map(|map| map.values()).collect::<Vec<&T>>()
    }

    // *********** PRIVATE ***********

    /// Impose boundary conditions for collisions
    fn impose_boundaries(&self, gx: i16, gy: i16) -> (usize, usize) {
        let mut ret: (usize, usize) = (0, 0);
        if gx < 0 {
            ret.0 = self.span.0 - 1;
        } else if gx >= self.span.0 as i16 {
            ret.0 = 0; 
        } else {
            ret.0 = gx as usize;
        }

        if gy < 0{
            ret.1 = self.span.1 - 1;
        } else if gy >= self.span.1 as i16 {
            ret.1 = 0;
        } else{
            ret.1 = gy as usize;
        }
        ret
    }

    fn euclidean_dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return ((x1-x2).powi(2) + (y1-y2).powi(2)).sqrt()
    }


}

impl<T: Spatial2D> Grid<T>{
    /// Push an object to the grid
    pub fn push(&mut self, obj: T){
        let x = obj.x();
        let y = obj.y();
        let objid = obj.id();

        //println!("{:}, {:}", x, y);

        let gidx = ( x / self.spacing.0 ) as usize;
        let gidy = ( y / self.spacing.1 ) as usize;
        let gid = gidx + gidy * self.span.0 ; 

        self.mapvec[gid].insert(objid, obj);
        self.idlookup.insert(objid, gid);
    }
    /// Move an object in x and y (pull out object, change its x/y and push it back)
    pub fn move_obj(&mut self, objid: u32, x_inc: f32, y_inc: f32){

        // Remove the object from the grid
        let mut obj = self.remove(objid);

        // Move object (obj increment functions handling wrapping/reflection at walls)
        obj.x_inc(x_inc, 100.0);
        obj.y_inc(y_inc, 100.0);

        // Push it back into the Grid
        self.push(obj);
    }

    /// Resolve all collisions between objects.
    pub fn detect_and_resolve_collisions(&mut self) {

        // A hashmap containing all collisions to be resolved (  -> (dx,dy) )
        let mut col_to_resolve: AHashMap< u32, (f32, f32) > = AHashMap::new();

        // Detect all collisions and add to col_to_resolve hashmap
        for (objid, gid) in self.idlookup.iter(){
            let gx = (*gid % self.span.0) as i16;
            let gy  = (*gid / self.span.0) as i16;

            // Calculate coordinates of neighboring grid cells
            let neighbor_coords = [ (gx-1, gy+1), (gx, gy+1), (gx+1, gy+1),
                                                            (gx-1, gy)  , (gx, gy)  , (gx+1, gy)  ,
                                                            (gx-1, gy-1), (gx, gy-1), (gx+1, gy-1)];
            
            // For each neighboring grid cell
            for (nx, ny) in neighbor_coords {
                let (nx, ny) = self.impose_boundaries(nx, ny);

                let ngid = nx + ny * self.span.0 ;

                // Don't recheck collisions with the previous cell!
                // For each object in this neighboring grid cell (the "neighbor object")
                for n_objid in self.mapvec[ngid].keys(){

                    // Given the neighbor object is not the same as the original object
                    if n_objid > objid {

                        let ((x1,y1), dia1) = self.mapvec[*gid].get(objid).unwrap().spatial_props();
                        let ((x2,y2), dia2) = self.mapvec[ngid].get(n_objid).unwrap().spatial_props();

                        // Calculate distance between the center of the objects and sum of radii
                        let dist = Grid::<T>::euclidean_dist(x1, y1, x2, y2);
                        let sum_radii = (dia1/2.0) + (dia2/2.0);

                        // Check if object and its neighbor are colliding (dist < sum of radii)
                        if dist <= sum_radii {
                            // If distance between centers of objects is less than the sum of radii they are colliding!
                            // Each object is to be moved by (sum of radii - dist) / 2.0
                            let d= (sum_radii - dist) / 2.0;

                            let move_x = (d) * ((x2-x1)/dist);
                            let move_y = (d) * ((y2-y1)/dist);
                            
                            // Add future collision modifier to objid
                            if col_to_resolve.contains_key(objid){
                                col_to_resolve.get_mut(objid).unwrap().0 -= move_x;
                                col_to_resolve.get_mut(objid).unwrap().1 -= move_y;
                            } else {
                                col_to_resolve.insert(*objid, (-move_x, -move_y));
                            }

                            // Add future collision modifier to objid
                            if col_to_resolve.contains_key(objid){
                                col_to_resolve.get_mut(objid).unwrap().0 -= move_x;
                                col_to_resolve.get_mut(objid).unwrap().1 -= move_y;
                            } else {
                                col_to_resolve.insert(*objid, (-move_x, -move_y));
                            }

                            // Add future collision modifier to neighbor
                            if col_to_resolve.contains_key(n_objid){
                                col_to_resolve.get_mut(n_objid).unwrap().0 += move_x;
                                col_to_resolve.get_mut(n_objid).unwrap().1 += move_y;
                            } else {
                                col_to_resolve.insert(*n_objid, (move_x, move_y));
                            }
                        }
                    }
                }
            }
        }

        // Resolve all detected collisions by iterating over col_to_resolve
        for (objid, (dx, dy)) in col_to_resolve.drain(){
            self.move_obj(objid, dx, dy);
        }
    }

    // *********** PRIVATE ***********
    
}
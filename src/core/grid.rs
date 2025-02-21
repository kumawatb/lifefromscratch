use ahash::AHashMap;
use crate::core::atom::Spatial2D;

/// This struct is essentially a container for the atoms with its own spatial hashing implementation
pub struct Grid<T>{
    mapvec: Vec<AHashMap<u32,T>>,
    idlookup: AHashMap<u32, usize>,
    spacing: (f32, f32),
    span: (usize, usize)
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


    /// Push an object to the grid at a given x,y
    pub fn push(&mut self, obj: T, x: f32, y: f32, objid: u32){
        let gidx = (x/self.spacing.0) as usize;
        let gidy = (y/self.spacing.1) as usize;
        let gid = gidx + gidy * self.span.0 ; 

        self.mapvec[gid].insert(objid, obj);
        self.idlookup.insert(objid, gid);
    }



    pub fn iter(&self) -> Vec<&T> {
        self.mapvec.iter().flat_map(|map| map.values()).collect::<Vec<&T>>()
    }


}

impl<T: Spatial2D> Grid<T>{
    /// Move an object in x and y
    pub fn move_obj(&mut self, objid: u32, x_inc: f32, y_inc: f32){
        let obj = self.mapvec[*self.idlookup.get(&objid).unwrap()].get_mut(&objid).unwrap();
        obj.x_inc(x_inc, 100.0);
        obj.y_inc(y_inc, 100.0);
    }
}
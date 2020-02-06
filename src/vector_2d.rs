pub struct Vector2D{
    pub x : f32,
    pub y : f32,
}

#[allow(dead_code)]
impl Vector2D {
    pub fn dot (&self, vector: &Vector2D) -> f32 {
        (self.normalized().x * vector.normalized().x) //x component
        +
        (self.normalized().y * vector.normalized().y) //y component
    }

    pub fn dot_fast (&self, vector: &Vector2D) -> f32 {
        (self.x * vector.x) //x component
        +
        (self.y * vector.y) //y component
    }

    pub fn length (&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    pub fn normalized (&self) -> Vector2D{
        let length = self.length();
        
        if length == 0.0 {
            Vector2D { x: 0.0, y: 0.0 }
        }else{
            Vector2D { x: self.x/length, y: self.y/length }
        }

    }
}

impl Clone for Vector2D {
    fn clone(&self) -> Self {
        Vector2D{ x: self.x, y: self.y }
    }
}
use crate::basic_types::{Vector3, Ray, Hit,};

pub trait Renderable {
    fn get_id(&self) -> String;
    fn get_normal_at(&self, point: Vector3) -> Vector3;
    fn get_intersection(&self, ray: &Ray) -> Option<Hit>;
}

// class RenderObject {
//     public:
//         RenderObject(Vec position, string id);
//         RenderObject(Vec position, string id, RenderMaterial material);
//         virtual ~RenderObject() {}
    
//         const RenderMaterial& getMaterial() const;
//         const string& getId() const;
//         const Vec& getPosition() const;
//         virtual Hit getIntersection(const Ray& ray) const = 0;
//         virtual Vec getNormalAtPoint(const Vec& point) const = 0;
//         virtual Pixel getExplicitLightContribution(const Ray& ray, const Hit& hitPoint) const;
//     protected:
//         RenderMaterial material;
//         Vec position;
//         string id;
//     };
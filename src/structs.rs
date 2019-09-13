use serde::
{
    Deserialize,
};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Element
{
    pub name: Option<String>, 
    pub appearance: Option<String>, 
    pub phase: Option<String>, 
    pub source: Option<String>, 
    pub spectral_img: Option<String>, 
    pub summary: Option<String>,
    pub symbol: Option<String>, 
    pub category: Option<String>, 
    pub discovered_by: Option<String>, 
    pub color: Option<String>, 
    pub named_by: Option<String>, 
    pub electron_configuration: Option<String>,
    pub atomic_mass: Option<f64>,
    pub boil: Option<f64>, 
    pub density: Option<f64>, 
    pub melt: Option<f64>, 
    pub molar_heat: Option<f64>, 
    pub electron_affinity: Option<f64>,
    pub electronegativity_pauling: Option<f64>,
    pub number: Option<u32>, 
    pub period: Option<u32>, 
    pub xpos: Option<u32>, 
    pub ypos: Option<u32>, 
    pub ionization_energies: Option<Vec<f64>>,
    pub shells: Option<Vec<u32>>,
}
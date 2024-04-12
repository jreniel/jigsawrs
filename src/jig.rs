use libjigsaw_sys::jigsaw_init_jig_t;
use libjigsaw_sys::jigsaw_jig_t;

pub enum JigsawHfunScale {
    RELATIVE,
    ABSOLUTE,
}

pub struct JigsawJig {
    config: jigsaw_jig_t,
}

impl JigsawJig {
    pub fn new() -> Self {
        let mut config: jigsaw_jig_t = unsafe { std::mem::zeroed() };
        unsafe {
            jigsaw_init_jig_t(&mut config);
        }
        Self { config }
    }
    pub fn set_verbosity(&mut self, verbosity: i32) -> &mut Self {
        self.config._verbosity = verbosity;
        self
    }
    pub fn set_numthread(&mut self, numthread: i32) -> &mut Self {
        self.config._numthread = numthread;
        self
    }
    pub fn set_geom_seed(&mut self, geom_seed: i32) -> &mut Self {
        self.config._geom_seed = geom_seed;
        self
    }
    pub fn set_geom_feat(&mut self, geom_feat: i32) -> &mut Self {
        self.config._geom_feat = geom_feat;
        self
    }
    pub fn set_geom_eta1(&mut self, geom_eta1: f64) -> &mut Self {
        self.config._geom_eta1 = geom_eta1;
        self
    }
    pub fn set_geom_eta2(&mut self, geom_eta2: f64) -> &mut Self {
        self.config._geom_eta2 = geom_eta2;
        self
    }
    pub fn set_init_near(&mut self, init_near: f64) -> &mut Self {
        self.config._init_near = init_near;
        self
    }
    pub fn set_hfun_scal(&mut self, hfun_scal: JigsawHfunScale) -> &mut Self {
        match hfun_scal {
            JigsawHfunScale::RELATIVE => {
                self.config._hfun_scal = libjigsaw_sys::JIGSAW_HFUN_RELATIVE as i32;
            }
            JigsawHfunScale::ABSOLUTE => {
                self.config._hfun_scal = libjigsaw_sys::JIGSAW_HFUN_ABSOLUTE as i32;
            }
        }
        self
    }
    pub fn set_hfun_hmax(&mut self, hfun_hmax: f64) -> &mut Self {
        self.config._hfun_hmax = hfun_hmax;
        self
    }
    pub fn set_hfun_hmin(&mut self, hfun_hmin: f64) -> &mut Self {
        self.config._hfun_hmin = hfun_hmin;
        self
    }
    pub fn set_bnds_kern(&mut self, bnds_kern: i32) -> &mut Self {
        self.config._bnds_kern = bnds_kern;
        self
    }

    pub fn set_mesh_dims(&mut self, mesh_dims: i32) -> &mut Self {
        self.config._mesh_dims = mesh_dims;
        self
    }

    pub fn set_mesh_kern(&mut self, mesh_kern: i32) -> &mut Self {
        self.config._mesh_kern = mesh_kern;
        self
    }

    pub fn set_mesh_iter(&mut self, mesh_iter: u32) -> &mut Self {
        self.config._mesh_iter = mesh_iter as i32;
        self
    }

    pub fn set_mesh_top1(&mut self, mesh_top1: i32) -> &mut Self {
        self.config._mesh_top1 = mesh_top1;
        self
    }

    pub fn set_mesh_top2(&mut self, mesh_top2: i32) -> &mut Self {
        self.config._mesh_top2 = mesh_top2;
        self
    }

    pub fn set_mesh_rad2(&mut self, mesh_rad2: f64) -> &mut Self {
        self.config._mesh_rad2 = mesh_rad2;
        self
    }

    pub fn set_mesh_rad3(&mut self, mesh_rad3: f64) -> &mut Self {
        self.config._mesh_rad3 = mesh_rad3;
        self
    }
    pub fn set_mesh_siz1(&mut self, mesh_siz1: f64) -> &mut Self {
        self.config._mesh_siz1 = mesh_siz1;
        self
    }

    pub fn set_mesh_siz2(&mut self, mesh_siz2: f64) -> &mut Self {
        self.config._mesh_siz2 = mesh_siz2;
        self
    }

    pub fn set_mesh_siz3(&mut self, mesh_siz3: f64) -> &mut Self {
        self.config._mesh_siz3 = mesh_siz3;
        self
    }

    pub fn set_mesh_off2(&mut self, mesh_off2: f64) -> &mut Self {
        self.config._mesh_off2 = mesh_off2;
        self
    }

    pub fn set_mesh_off3(&mut self, mesh_off3: f64) -> &mut Self {
        self.config._mesh_off3 = mesh_off3;
        self
    }

    pub fn set_mesh_snk2(&mut self, mesh_snk2: f64) -> &mut Self {
        self.config._mesh_snk2 = mesh_snk2;
        self
    }

    pub fn set_mesh_snk3(&mut self, mesh_snk3: f64) -> &mut Self {
        self.config._mesh_snk3 = mesh_snk3;
        self
    }
    pub fn set_mesh_eps1(&mut self, mesh_eps1: f64) -> &mut Self {
        self.config._mesh_eps1 = mesh_eps1;
        self
    }

    pub fn set_mesh_eps2(&mut self, mesh_eps2: f64) -> &mut Self {
        self.config._mesh_eps2 = mesh_eps2;
        self
    }

    pub fn set_mesh_vol3(&mut self, mesh_vol3: f64) -> &mut Self {
        self.config._mesh_vol3 = mesh_vol3;
        self
    }

    pub fn set_optm_kern(&mut self, optm_kern: i32) -> &mut Self {
        self.config._optm_kern = optm_kern;
        self
    }

    pub fn set_optm_iter(&mut self, optm_iter: i32) -> &mut Self {
        self.config._optm_iter = optm_iter;
        self
    }

    pub fn set_optm_cost(&mut self, optm_cost: i32) -> &mut Self {
        self.config._optm_cost = optm_cost;
        self
    }

    pub fn set_optm_beta(&mut self, optm_beta: f64) -> &mut Self {
        self.config._optm_beta = optm_beta;
        self
    }

    pub fn set_optm_zeta(&mut self, optm_zeta: f64) -> &mut Self {
        self.config._optm_zeta = optm_zeta;
        self
    }

    pub fn set_optm_qtol(&mut self, optm_qtol: f64) -> &mut Self {
        self.config._optm_qtol = optm_qtol;
        self
    }

    pub fn set_optm_qlim(&mut self, optm_qlim: f64) -> &mut Self {
        self.config._optm_qlim = optm_qlim;
        self
    }

    pub fn set_optm_tria(&mut self, optm_tria: i32) -> &mut Self {
        self.config._optm_tria = optm_tria;
        self
    }

    pub fn set_optm_dual(&mut self, optm_dual: i32) -> &mut Self {
        self.config._optm_dual = optm_dual;
        self
    }

    pub fn set_optm_zip_(&mut self, optm_zip_: i32) -> &mut Self {
        self.config._optm_zip_ = optm_zip_;
        self
    }

    pub fn set_optm_div_(&mut self, optm_div_: i32) -> &mut Self {
        self.config._optm_div_ = optm_div_;
        self
    }
}

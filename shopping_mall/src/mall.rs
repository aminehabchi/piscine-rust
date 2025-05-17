use std::collections::HashMap;

#[inline]
fn coerce_map<V>(m: HashMap<impl Into<String>, V>) -> HashMap<String, V> {
    m.into_iter().map(|(k, v)| (k.into(), v)).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    pub name: String,
    pub guards: HashMap<String, Guard>,
    pub floors: HashMap<String, Floor>,
}

impl Mall {
    pub fn new(
        name: impl Into<String>,
        guards: HashMap<impl Into<String>, Guard>,
        floors: HashMap<impl Into<String>, Floor>,
    ) -> Self {
        Self {
            name: name.into(),
            guards: coerce_map(guards),
            floors: coerce_map(floors),
        }
    }

    pub fn change_name(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }

    pub fn hire_guard(&mut self, name: impl Into<String>, guard: Guard) {
        self.guards.insert(name.into(), guard);
    }

    pub fn fire_guard(&mut self, name: impl Into<String>) {
        self.guards.remove(&name.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guard {
    pub age: u32,
    pub years_experience: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Floor {
    pub stores: HashMap<String, Store>,
    pub size_limit: u64,
}

impl Floor {
    pub fn new(stores: HashMap<impl Into<String>, Store>, size_limit: u64) -> Self {
        Self {
            stores: coerce_map(stores),
            size_limit,
        }
    }

    pub fn replace_store(&mut self, store: impl Into<String>, with: Store) {
        self.stores.entry(store.into()).and_modify(|v| *v = with);
    }

    pub fn add_store(&mut self, name: impl Into<String>, store: Store) -> Result<(), ()> {
        let has_space = self.size_limit
            >= self.stores.values().map(|s| s.square_meters).sum::<u64>() + store.square_meters;

        if has_space {
            self.stores.insert(name.into(), store);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn remove_store(&mut self, name: impl Into<String>) {
        self.stores.remove(&name.into());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub employees: HashMap<String, Employee>,
    pub square_meters: u64,
}

impl Store {
    pub fn new(employees: HashMap<impl Into<String>, Employee>, square_meters: u64) -> Self {
        Self {
            employees: coerce_map(employees),
            square_meters,
        }
    }

    pub fn hire_employee(&mut self, name: impl Into<String>, employee: Employee) {
        self.employees.insert(name.into(), employee);
    }

    pub fn fire_employee(&mut self, name: impl Into<String>) {
        self.employees.remove(&name.into());
    }

    pub fn expand(&mut self, by: u64) {
        self.square_meters += by;
    }
}

#[derive(Debug, Copy,Clone, PartialEq)]
pub struct Employee {
    pub age: u32,
    pub working_hours: (u32, u32),
    pub salary: f64,
}

impl Employee {
    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn change_workload(&mut self, from: u32, to: u32) {
        self.working_hours = (from, to);
    }

    pub fn raise(&mut self, amount: f64) {
        self.salary += amount;
    }

    pub fn cut(&mut self, amount: f64) {
        self.salary -= amount;
    }
}

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max: u64 = 0;
    let mut result: (String, Store) = ("".to_string(), Store {
        employees: HashMap::new(),
        square_meters: 0,
    });

    for (_floor_name, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            if store.square_meters > max {
                max = store.square_meters;
                result = (store_name.clone(), store.clone());
            }
        }
    }

    result
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut max_salary = 0.0;
    let mut name: String = "".to_owned();
    let mut employee: Employee = Employee {
        age: 0,
        working_hours: (0, 0),
        salary: 0.0,
    };

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            for (employee_name, empl) in &store.employees {
                if empl.salary > max_salary {
                    max_salary = empl.salary;
                    employee = empl.clone();
                    name = employee_name.clone();
                }
            }
        }
    }

    vec![(name, employee)]
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut total = mall.guards.len();

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            total += store.employees.len();
        }
    }

    total
}

pub fn check_for_securities(mall: &mut Mall, mut guards: HashMap<String, Guard>) {
    let mut total_area: usize = 0;

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            total_area += store.square_meters as usize;
        }
    }

    let required_guards = (total_area + 199) / 200;
    let current_guards = mall.guards.len();

    if current_guards < required_guards {
        let needed = required_guards - current_guards;
        for (k, v) in guards.drain() {
            mall.guards.insert(k, v);
            if mall.guards.len() >= required_guards {
                break;
            }
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_floor_name, floor) in &mut mall.floors {
        for (_store_name, store) in &mut floor.stores {
            for (_employee_name, empl) in &mut store.employees {
                let work_hours = empl.working_hours.1 as i32 - empl.working_hours.0 as i32;
                if work_hours > 10 {
                    empl.raise(empl.salary * 0.1);
                } else {
                    empl.cut(empl.salary * 0.1);
                }
            }
        }
    }
}

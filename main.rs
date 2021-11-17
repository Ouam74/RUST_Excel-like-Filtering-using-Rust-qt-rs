#![windows_subsystem = "windows"]
#![allow(unused)]

use cpp_core::{Ptr, Ref, StaticUpcast,};
use cpp_core::{CppBox};
use qt_core::{
    q_init_resource, qs, slot, ScrollBarPolicy, CheckState, QVectorOf, QBox, QObject, QPtr, QPointF, SlotOfInt,
    SlotNoArgs, QFlags, WindowType, WindowModality, AlignmentFlag, QCoreApplication, Orientation, QStringList,
    q_state_machine::SignalEvent, QListOfQVariant, QItemSelectionModel, QModelIndex, QListOfQModelIndex, QVariant, QString,
};
use qt_gui::{QStandardItem, QStandardItemModel, QPen, QBrush, QColor, q_painter::RenderHint, SignalOfQStandardItem, SlotOfQStandardItem,
};
use qt_widgets::{q_abstract_scroll_area, QApplication, QMainWindow, QListOfQWidget, QTableWidget, QTableWidgetItem,
     QHeaderView, QPushButton, QWidget, QFrame, QVBoxLayout, QGridLayout, QTreeView, QHBoxLayout,
 };
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use itertools::Itertools;
use std::ptr;

///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    gridlayout: QBox<QGridLayout>,
    treeview: QBox<QTreeView>,
    verticallayout: QBox<QVBoxLayout>,
    horizontallayout: QBox<QHBoxLayout>,
    enterbutton: QBox<QPushButton>,
    cancelbutton: QBox<QPushButton>,
}

impl StaticUpcast<QObject> for Form {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Form {
    fn new() -> Rc<Form> {
        unsafe {
          let widget = QWidget::new_0a();
          widget.set_window_flags(QFlags::from(WindowType::FramelessWindowHint));
          widget.set_window_modality(WindowModality::ApplicationModal);
          let gridlayout = QGridLayout::new_1a(&widget);
          let verticallayout = QVBoxLayout::new_0a();
          let treeview = QTreeView::new_1a(&widget);
          verticallayout.add_widget_1a(&treeview);
          let horizontallayout = QHBoxLayout::new_0a();
          let enterbutton = QPushButton::new();
          enterbutton.set_text(&qs("Enter"));
          horizontallayout.add_widget(&enterbutton);
          let cancelbutton = QPushButton::new();
          cancelbutton.set_text(&qs("Cancel"));
          horizontallayout.add_widget(&cancelbutton);
          verticallayout.add_layout_1a(&horizontallayout);
          gridlayout.add_layout_5a(&verticallayout, 0, 0, 1, 1);

          // widget.show();

          let this = Rc::new(Self {
              widget,
              gridlayout,
              verticallayout,
              treeview,
              horizontallayout,
              enterbutton,
              cancelbutton,
          });
          this.init();
          this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        // self.treeview.slot_data_changed().connect(&self.slot_on_item_changed());
        self.enterbutton.clicked().connect(&self.slot_on_enter_clicked());
        self.cancelbutton.clicked().connect(&self.slot_on_cancel_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_enter_clicked(self: &Rc<Self>) {
        println!("enter clicked");
        // Filtering function must be located here
        self.widget.close();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_cancel_clicked(self: &Rc<Self>) {
        println!("cancel clicked");
        self.widget.close();
    }

    // #[slot(SlotNoArgs)]
    // unsafe fn on_item_changed(self: &Rc<Self>) {
    //     println!("item changed");
    // }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct MyWindow {
    window: QBox<QMainWindow>,
    centralwidget: QBox<QWidget>,
    gridlayout: QBox<QGridLayout>,
    table: QBox<QTableWidget>,
    verticallayout: QBox<QVBoxLayout>,
    button_vec: Vec<Ptr<QPushButton>>,
    model_vec: Vec<Ptr<QStandardItemModel>>,
    form_vec: Vec<Rc<Form>>,
    col: i32,
    row: i32,
    form_index: Arc<Mutex<i32>>,
    old_vec: Arc<Mutex<Vec<Ptr<QVariant>>>>,
    new_vec: Arc<Mutex<Vec<Ptr<QVariant>>>>,
}

impl StaticUpcast<QObject> for MyWindow {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.window.as_ptr().static_upcast()
    }
}

impl MyWindow {
    fn new() -> Rc<MyWindow> {
        unsafe {
           let row: i32 = 11;
           let col: i32 = 3;
           let form_index = Arc::new(Mutex::new(0));
           let window = QMainWindow::new_0a();
           window.resize_2a(400, 400);
           let centralwidget = QWidget::new_1a(&window);
           let gridlayout = QGridLayout::new_1a(&centralwidget);
           let verticallayout = QVBoxLayout::new_0a();
           gridlayout.add_layout_5a(&verticallayout, 0, 0, 2, 2);
           window.set_central_widget(&centralwidget);
           let table = QTableWidget::new_0a();
           verticallayout.add_widget(&table);
           table.set_row_count(row);
           table.set_column_count(col);
           table.horizontal_header().set_stretch_last_section(true);
           table.vertical_header().set_stretch_last_section(true);
           table.set_alternating_row_colors(true);
           let new_vec = Arc::new(Mutex::new(Vec::new()));
           let old_vec = Arc::new(Mutex::new(Vec::new()));

           let my_string_array = [
                            ["Column", "Unit", "Test Group"],
                            ["A", "1", "a"],
                            ["A", "1", "b"],
                            ["A", "1", "c"],
                            ["A", "2", "a"],
                            ["B", "1", "a"],
                            ["B", "2", "c"],
                            ["B", "3", "a"],
                            ["C", "1", "a"],
                            ["C", "2", "c"],
                            ["D", "1", "c"]
            ];

            // Fill in table
            for i in 1..row {
               for j in 0..col {
                   let item = QTableWidgetItem::new().into_ptr();
                   item.set_text(&qs(my_string_array[i as usize][j as usize].to_string()));
                   table.set_item(i, j, item);
               }
            }
            // Create choices array
            let mut choices_2d: Vec<Vec<String>> = Vec::new();
            for i in 0..col {
                let mut my_col: Vec<String> = Vec::new();
                for j in 1..row {
                    my_col.push(my_string_array[j as usize][i as usize].to_string());
                }
                let choices = my_col.iter().cloned().unique().collect_vec();
                choices_2d.push(choices);
            }
            // Create QPushButton array and place QPushButton in the cell
            let mut button_vec = Vec::new();
            for i in 0..col {
                let button = QPushButton::new().into_ptr();
                button.set_object_name((&qs(i.to_string())));
                button.set_text(&qs(my_string_array[0][i as usize]));
                table.set_cell_widget(0,i,button);
                button_vec.push(button);
           }
           // Create QStandardItemModel array
           let mut model_vec = Vec::new();
           for i in 0..col {
                let model = QStandardItemModel::new_0a().into_ptr();
                model_vec.push(model);
           }
           // Fill in the QStandardItemModel(s) with choices items
           for i in 0..col {
                let nrow = choices_2d[i as usize].len();
                let item = QStandardItem::new().into_ptr();
                item.set_text(&qs("Select All".to_string()));
                item.set_checkable(true);
                item.set_check_state(CheckState::Checked);
                model_vec[i as usize].append_row_q_standard_item(item);
                for j in 0..nrow {
                    let item = QStandardItem::new().into_ptr();
                    item.set_text(&qs(choices_2d[i as usize][j as usize].to_string()));
                    item.set_checkable(true);
                    item.set_check_state(CheckState::Checked);
                    model_vec[i as usize].append_row_q_standard_item(item);
                }
            }
            // // Create current check state value of the "Select All" items
            // for i in 0..col {
            //      // let nrow = model_vec[i as usize].len();
            //      let mut item = QStandardItem::new().into_ptr();
            //      // let variant = QVariant::new().into_ptr();
            //      item = model_vec[i as usize].item_1a(0);
            //      *old_vec.append(item.check_state());
            //      // *old_vec[i as usize].lock().unwrap() = item.check_state();
            //      println!("{:?}", old_vec[i as usize]);
            //  }
            // Create Form array
            let mut form_vec = Vec::new();
            for i in 0..col {
                let form = Form::new();
                form.treeview.set_model(model_vec[i as usize]);
                form_vec.push(form);
            }

           window.show();

           let this = Rc::new(Self {
             window,
             centralwidget,
             gridlayout,
             table,
             verticallayout,
             button_vec,
             model_vec,
             form_vec,
             old_vec,
             new_vec,
             col,
             row,
             form_index,
           });
           this.init();
           this
         }
    }

    unsafe fn init(self: &Rc<Self>) {
        for i in 0..self.col {
            self.button_vec[i as usize].clicked().connect(&self.slot_on_button_clicked());
            self.model_vec[i as usize].item_changed().connect(&self.slot_on_item_changed());
        }
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_button_clicked(self: &Rc<Self>) {
        *self.form_index.lock().unwrap() = self.table.current_column() as i32;
        let form_index = *self.form_index.lock().unwrap();
        self.form_vec[form_index as usize].widget.show();
    }

    // #[slot(SlotNoArgs)]
    #[slot(SlotOfQStandardItem)]
    unsafe fn on_item_changed(self: &Rc<Self>, item_clicked: Ptr<QStandardItem>) {
         let form_index: i32 = *self.form_index.lock().unwrap();
         if item_clicked.text().to_std_string() == "Select All".to_string() && item_clicked.check_state() == CheckState::Unchecked
         {
            let mut item = QStandardItem::new().into_ptr();
            for i in 0..self.model_vec[form_index as usize].row_count_0a()
            {
                 item = self.model_vec[form_index as usize].item_1a(i);
                 item.set_check_state(CheckState::Unchecked);
            }
         }
         if item_clicked.text().to_std_string() == "Select All".to_string() && item_clicked.check_state() == CheckState::Checked
         {
            let mut item = QStandardItem::new().into_ptr();
            for i in 0..self.model_vec[form_index as usize].row_count_0a()
            {
                item = self.model_vec[form_index as usize].item_1a(i);
                item.set_check_state(CheckState::Checked);
            }
         }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    QApplication::init(|_| unsafe {
        let todo_widget = MyWindow::new();
        unsafe { QApplication::exec() }
    })
}

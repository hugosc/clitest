use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};
use crate::app::state::{AppState, AppMode};
use crate::ui::modal::InputField;

pub fn render(frame: &mut Frame, state: &AppState) {
    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    // Render list on the left
    render_list(frame, state, chunks[0]);

    // Render details on the right
    render_details(frame, state, chunks[1]);

    // Render any active modal or error on top
    if let Some(err) = &state.error_message {
        render_error_popup(frame, err);
    } else if state.mode == AppMode::Help {
        render_help_modal(frame);
    } else if state.mode == AppMode::ConfirmDelete {
        render_delete_confirm_modal(frame);
    } else if state.mode == AppMode::Filter {
        render_filter_input(frame, state);
    } else if state.mode == AppMode::AddFruit || state.mode == AppMode::EditFruit {
        if let Some(modal) = &state.modal {
            let title = if state.mode == AppMode::AddFruit {
                "Add Fruit"
            } else {
                "Edit Fruit"
            };
            render_fruit_modal(frame, modal, title);
        }
    }
}

fn render_list(frame: &mut Frame, state: &AppState, area: Rect) {
    let display_fruits = state.display_fruits();
    let items: Vec<ListItem> = display_fruits
        .iter()
        .map(|f| ListItem::new(f.name.as_str()))
        .collect();

    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(state.selected_index));

    let title = if state.is_filtering() {
        format!(
            "Fruits — (filtered: {}/{}) [/] search, [a] add, [e] edit, [d] delete, [Esc] clear",
            display_fruits.len(),
            state.fruits.len()
        )
    } else {
        "Fruits — [↑/↓/j/k] navigate, [a] add, [e] edit, [d] delete, [/] search, [?] help"
            .to_string()
    };

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_details(frame: &mut Frame, state: &AppState, area: Rect) {
    let details = if state.fruits.is_empty() {
        Paragraph::new("No fruits available")
            .block(Block::default().title("Details").borders(Borders::ALL))
    } else if let Some(fruit) = state.selected_fruit() {
        let txt = format!(
            "Name: {}\n\nDimensions:\n  Length: {}\n  Width : {}\n  Height: {}\n\nVolume: {:.2}",
            fruit.name, fruit.length, fruit.width, fruit.height, fruit.volume()
        );
        Paragraph::new(txt).block(
            Block::default()
                .title(format!("Details [{}]", state.selected_index + 1))
                .borders(Borders::ALL),
        )
    } else {
        Paragraph::new("Select a fruit")
            .block(Block::default().title("Details").borders(Borders::ALL))
    };

    frame.render_widget(details, area);
}

fn render_filter_input(frame: &mut Frame, state: &AppState) {
     let popup_area = centered_rect(60, 15, frame.area());

     frame.render_widget(Clear, popup_area);
     
     // Create a search prompt with the current query
     let lines = vec![
         Line::from(vec![
             Span::raw("Enter search query ("),
             Span::styled("Esc", Style::default().fg(Color::Red)),
             Span::raw(" to cancel, "),
             Span::styled("Enter", Style::default().fg(Color::Green)),
             Span::raw(" to confirm):"),
         ]),
         Line::from(""),
         Line::from(format!("> {}", state.filter_query)),
     ];
     
     let para = Paragraph::new(lines)
         .block(Block::default().title("Filter Fruits").borders(Borders::ALL))
         .alignment(Alignment::Left);

     frame.render_widget(para, popup_area);
 }

fn render_delete_confirm_modal(frame: &mut Frame) {
    let popup_area = centered_rect(50, 15, frame.area());

    frame.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from("Are you sure you want to delete this fruit?"),
        Line::from(""),
        Line::from(vec![
            Span::raw("["),
            Span::styled("Y", Style::default().fg(Color::Yellow)),
            Span::raw("]es  ["),
            Span::styled("N", Style::default().fg(Color::Yellow)),
            Span::raw("]o"),
        ]),
    ];

    let para = Paragraph::new(lines)
        .block(Block::default().title("Confirm Delete").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(para, popup_area);
}

fn render_fruit_modal(frame: &mut Frame, modal: &crate::ui::modal::ModalState, title: &str) {
     let frame_area = frame.area();
     
     // Make modal responsive to terminal size - use smaller percentages for small terminals
     let width_percent = if frame_area.width < 80 { 90 } else if frame_area.width < 120 { 75 } else { 60 };
     let height_percent = if frame_area.height < 20 { 80 } else if frame_area.height < 30 { 60 } else { 50 };
     
     let popup_area = centered_rect(width_percent, height_percent, frame_area);
     frame.render_widget(Clear, popup_area);

     // Create the outer border
     let border = Block::default()
         .title(title)
         .borders(Borders::ALL)
         .border_type(ratatui::widgets::BorderType::Rounded);
     frame.render_widget(border, popup_area);

     // Create inner area for content (inside the border with 1px padding)
     let inner_full = Layout::default()
         .direction(Direction::Vertical)
         .margin(1)
         .constraints([Constraint::Min(0)])
         .split(popup_area);
     
     let inner_area = inner_full[0];

       let inner = Layout::default()
           .direction(Direction::Vertical)
           .constraints([
               Constraint::Length(2),
               Constraint::Length(2),
               Constraint::Length(2),
               Constraint::Length(2),
               Constraint::Length(2),
           ])
           .split(inner_area);

       // Helper function to render an input field with manual borders
       let render_input_field = |frame: &mut Frame, area: Rect, label: &str, content: &str, focused: bool| {
           let style = if focused {
               Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
           } else {
               Style::default()
           };
           
           // Render border
           let border = Block::default()
               .title(label)
               .borders(Borders::ALL)
               .style(style);
           frame.render_widget(border, area);
           
           // Render content inside (leaving space for borders: 1px on each side)
           let content_area = Rect {
               x: area.x + 1,
               y: area.y + 1,
               width: area.width.saturating_sub(2),
               height: area.height.saturating_sub(2),
           };
           
           if content_area.width > 0 && content_area.height > 0 {
               let text_widget = Paragraph::new(content)
                   .style(style);
               frame.render_widget(text_widget, content_area);
           }
       };

       // Name field
       let focused_name = modal.focused_field == InputField::Name;
       render_input_field(frame, inner[0], "Name", modal.name.as_str(), focused_name);

       // Length field
       let focused_length = modal.focused_field == InputField::Length;
       render_input_field(frame, inner[1], "Length", modal.length.as_str(), focused_length);

       // Width field
       let focused_width = modal.focused_field == InputField::Width;
       render_input_field(frame, inner[2], "Width", modal.width.as_str(), focused_width);

       // Height field
       let focused_height = modal.focused_field == InputField::Height;
       render_input_field(frame, inner[3], "Height", modal.height.as_str(), focused_height);

     // Instructions
     let instructions = Line::from(vec![
         Span::raw("["),
         Span::styled("Tab", Style::default().fg(Color::Cyan)),
         Span::raw("] next  ["),
         Span::styled("S-Tab", Style::default().fg(Color::Cyan)),
         Span::raw("] prev  ["),
         Span::styled("Enter", Style::default().fg(Color::Green)),
         Span::raw("] save  ["),
         Span::styled("Esc", Style::default().fg(Color::Red)),
         Span::raw("] cancel"),
     ]);
     let instructions_widget = Paragraph::new(instructions)
         .block(Block::default().borders(Borders::ALL))
         .alignment(Alignment::Center);
     frame.render_widget(instructions_widget, inner[4]);

     // Error message if present
     if let Some(err) = &modal.error {
         let error_area = centered_rect(50, 15, frame.area());
         frame.render_widget(
             Paragraph::new(err.as_str())
                 .block(Block::default().title("Error").borders(Borders::ALL))
                 .style(Style::default().fg(Color::Red))
                 .alignment(Alignment::Center),
             error_area,
         );
     }
 }

fn render_error_popup(frame: &mut Frame, message: &str) {
    let popup_area = centered_rect(70, 20, frame.area());

    frame.render_widget(Clear, popup_area);

    let para = Paragraph::new(message)
        .block(Block::default().title("Error").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(para, popup_area);
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_help_modal(frame: &mut Frame) {
    let area = frame.area();
    let width = (area.width * 70) / 100;
    let height = (area.height * 80) / 100;
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    
    let popup_area = Rect {
        x,
        y,
        width,
        height,
    };

    // Create the help text
    let help_text = vec![
        Line::from(vec![Span::styled("KEYBOARD SHORTCUTS", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![Span::styled("Navigation", Style::default().fg(Color::Cyan))]),
        Line::from("  j/↓          - Move down"),
        Line::from("  k/↑          - Move up"),
        Line::from("  h/←          - Move left"),
        Line::from("  l/→          - Move right"),
        Line::from(""),
        Line::from(vec![Span::styled("Actions", Style::default().fg(Color::Cyan))]),
        Line::from("  /            - Filter by name"),
        Line::from("  a            - Add new fruit"),
        Line::from("  e            - Edit selected fruit"),
        Line::from("  d            - Delete selected fruit"),
        Line::from("  Ctrl+S       - Save changes"),
        Line::from(""),
        Line::from(vec![Span::styled("Modal Navigation", Style::default().fg(Color::Cyan))]),
        Line::from("  Tab          - Next field"),
        Line::from("  Shift+Tab    - Previous field"),
        Line::from("  Enter        - Confirm"),
        Line::from("  Esc          - Cancel/Back"),
        Line::from(""),
        Line::from(vec![Span::styled("Other", Style::default().fg(Color::Cyan))]),
        Line::from("  ?            - Show help (this screen)"),
        Line::from("  q            - Quit"),
        Line::from(""),
        Line::from(vec![Span::styled("Press Esc, q, or ? to close", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Black))
        )
        .alignment(Alignment::Left);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(paragraph, popup_area);
}
